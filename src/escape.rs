// This module implements escaping for various xml contexts

use core::{fmt, mem, str};

// All the routines here work only with and slice only at ascii characters
// This means conversion between `&str` and `&[u8]` is a noop even when slicing
#[inline]
fn from_utf8(v: &[u8]) -> &str {
	#[cfg(debug_assertions)]
	return str::from_utf8(v).unwrap();
	#[cfg(not(debug_assertions))]
	return unsafe { str::from_utf8_unchecked(v) };
}

// LLVM is big dum dum, trust me I'm a human
#[cfg(debug_assertions)]
macro_rules! unsafe_assert {
	($e:expr) => {};
}
#[cfg(not(debug_assertions))]
macro_rules! unsafe_assert {
	($e:expr) => { unsafe { if !$e { ::core::hint::unreachable_unchecked(); } } };
}

// Returns the index of the first one of `<`, `&`, `>`
// Returns the length of the input string if none found
#[inline]
fn split_text(bytes: &[u8]) -> usize {
	let mut i = 0;
	while i < bytes.len() {
		if bytes[i] == b'<' || bytes[i] == b'&' || bytes[i] == b'>' {
			break;
		}
		i += 1;
	}
	unsafe_assert!(i <= bytes.len());
	return i;
}

// Returns the index of the first one of `<`, `&`, `>`, `'`, `"`
// Returns the length of the input string if none found
#[inline]
fn split_attr(bytes: &[u8]) -> usize {
	let mut i = 0;
	while i < bytes.len() {
		if bytes[i] == b'<' || bytes[i] == b'&' || bytes[i] == b'>' || bytes[i] == b'\'' || bytes[i] == b'\"' {
			break;
		}
		i += 1;
	}
	unsafe_assert!(i <= bytes.len());
	return i;
}

// Writes the escaped character as a xml entity
// Writes nothing if chr is not one of `<`, `&`, `>`, `'`, `"`
#[inline]
fn escape_chr<W: ?Sized + fmt::Write>(write: &mut W, chr: u8) -> fmt::Result {
	match chr {
		b'<' => write.write_str(crate::obfstr!("&lt;")),
		b'&' => write.write_str(crate::obfstr!("&amp;")),
		b'>' => write.write_str(crate::obfstr!("&gt;")),
		b'\'' => write.write_str(crate::obfstr!("&apos;")),
		b'\"' => write.write_str(crate::obfstr!("&quot;")),
		_ => Ok(()),
	}
}

/// Escapes `<`, `&`, `>` when it appears in the formatted string.
///
/// # Examples
///
/// ```
/// fn check(input: &str, escaped: &str) {
/// 	let mut buf = String::new();
/// 	let mut writer = format_xml::EscapeText::wrap(&mut buf);
/// 	write!(writer, "{}", input).unwrap();
/// 	assert_eq!(buf, escaped);
/// }
///
/// check("", "");
/// check(" ", " ");
/// check("&", "&amp;");
/// check(" &", " &amp;");
/// check("& ", "&amp; ");
///
/// check("hello", "hello");
/// check("'world'", "\'world\'");
/// check("pre<script>post", "pre&lt;script&gt;post");
/// check("&<>\"\'", "&amp;&lt;&gt;\"\'");
/// check("&a'b<c>", "&amp;a\'b&lt;c&gt;");
/// ```
#[repr(transparent)]
pub struct EscapeText<T: ?Sized> {
	inner: T,
}
impl<T: ?Sized + fmt::Write> EscapeText<T> {
	#[inline]
	pub fn wrap(v: &mut T) -> &mut EscapeText<T> {
		unsafe { mem::transmute(v) }
	}
}
impl<T> From<T> for EscapeText<T> {
	#[inline]
	fn from(inner: T) -> Self { EscapeText { inner } }
}
// Forward Write calls
#[doc(hidden)]
impl<T: ?Sized + fmt::Write> EscapeText<T> {
	#[inline]
	pub fn write_str(&mut self, s: &str) -> fmt::Result {
		<Self as fmt::Write>::write_str(self, s)
	}
	#[inline]
	pub fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
		<Self as fmt::Write>::write_fmt(self, args)
	}
}
impl<T: ?Sized + fmt::Write> fmt::Write for EscapeText<T> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		let mut bytes = s.as_bytes();
		while bytes.len() > 0 {
			let i = split_text(bytes);
			let prefix = &bytes[..i];
			if prefix.len() > 0 {
				self.inner.write_str(from_utf8(prefix))?;
			}

			if let Some(&chr) = bytes.get(i) {
				escape_chr(&mut self.inner, chr)?;
				bytes = &bytes[i + 1..];
			}
			else {
				break;
			}
		}
		Ok(())
	}
}

/// Escapes `<`, `&`, `>`, `'`, `"` when it appears in the formatted string.
///
/// # Examples
///
/// ```
/// #[track_caller]
/// fn check(input: &str, escaped: &str) {
/// 	let mut buf = String::new();
/// 	let mut writer = format_xml::EscapeAttrValue::wrap(&mut buf);
/// 	writer.write_str(input).unwrap();
/// 	assert_eq!(buf, escaped);
/// }
///
/// check("", "");
/// check(" ", " ");
/// check("&", "&amp;");
/// check(" &", " &amp;");
/// check("& ", "&amp; ");
///
/// check("hello", "hello");
/// check("'world'", "&apos;world&apos;");
/// check("pre<script>post", "pre&lt;script&gt;post");
/// check("&<>\"\'", "&amp;&lt;&gt;&quot;&apos;");
/// check("&a'b<c>", "&amp;a&apos;b&lt;c&gt;");
/// ```
#[repr(transparent)]
pub struct EscapeAttrValue<T: ?Sized> {
	inner: T,
}
impl<T: ?Sized + fmt::Write> EscapeAttrValue<T> {
	#[inline]
	pub fn wrap(v: &mut T) -> &mut EscapeAttrValue<T> {
		unsafe { mem::transmute(v) }
	}
}
impl<T> From<T> for EscapeAttrValue<T> {
	#[inline]
	fn from(inner: T) -> Self {
		EscapeAttrValue { inner }
	}
}
// Forward Write calls
#[doc(hidden)]
impl<T: ?Sized + fmt::Write> EscapeAttrValue<T> {
	#[inline]
	pub fn write_str(&mut self, s: &str) -> fmt::Result {
		<Self as fmt::Write>::write_str(self, s)
	}
	#[inline]
	pub fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
		<Self as fmt::Write>::write_fmt(self, args)
	}
}
impl<T: ?Sized + fmt::Write> fmt::Write for EscapeAttrValue<T> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		let mut bytes = s.as_bytes();
		while bytes.len() > 0 {
			let i = split_attr(bytes);
			let prefix = &bytes[..i];
			if prefix.len() > 0 {
				self.inner.write_str(from_utf8(prefix))?;
			}

			if let Some(&chr) = bytes.get(i) {
				escape_chr(&mut self.inner, chr)?;
				bytes = &bytes[i + 1..];
			}
			else {
				break;
			}
		}
		Ok(())
	}
}

/// Escapes `--` in comments by not writing it at all.
///
/// # Safety
///
/// This implementation is incorrect, it won't catch callers who split `--` in multiple `write_str` calls!
#[doc(hidden)]
#[repr(transparent)]
pub struct EscapeComment<T: ?Sized> {
	inner: T,
}
impl<T: ?Sized + fmt::Write> EscapeComment<T> {
	#[inline]
	pub fn wrap(v: &mut T) -> &mut EscapeComment<T> {
		unsafe { mem::transmute(v) }
	}
}
// Forward Write calls
#[doc(hidden)]
impl<T: ?Sized + fmt::Write> EscapeComment<T> {
	#[inline]
	pub fn write_str(&mut self, s: &str) -> fmt::Result {
		<Self as fmt::Write>::write_str(self, s)
	}
	#[inline]
	pub fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
		<Self as fmt::Write>::write_fmt(self, args)
	}
}
impl<T: ?Sized + fmt::Write> fmt::Write for EscapeComment<T> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		for piece in s.split(crate::obfstr!("--")) {
			self.inner.write_str(piece)?;
		}
		Ok(())
	}
}

/// Escapes `]]>` in CDATA sections.
///
/// # Safety
///
/// This implementation is incorrect, it won't catch callers who split writing `]]>` in multiple `write_str` calls!
#[doc(hidden)]
#[repr(transparent)]
pub struct EscapeCharData<T: ?Sized> {
	inner: T,
}
impl<T: ?Sized + fmt::Write> EscapeCharData<T> {
	#[inline]
	pub fn wrap(v: &mut T) -> &mut EscapeCharData<T> {
		unsafe { mem::transmute(v) }
	}
}
// Forward Write calls
#[doc(hidden)]
impl<T: ?Sized + fmt::Write> EscapeCharData<T> {
	#[inline]
	pub fn write_str(&mut self, s: &str) -> fmt::Result {
		<Self as fmt::Write>::write_str(self, s)
	}
	#[inline]
	pub fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
		<Self as fmt::Write>::write_fmt(self, args)
	}
}
impl<T: ?Sized + fmt::Write> fmt::Write for EscapeCharData<T> {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		let mut write_sep = false;
		for piece in s.split(crate::obfstr!("]]>")) {
			if write_sep {
				self.inner.write_str(crate::obfstr!("]]]]><![CDATA[>"))?;
			}
			write_sep = true;
			self.inner.write_str(piece)?;
		}
		Ok(())
	}
}

#[test]
fn test_comment() {
	#[track_caller]
	fn check(input: &str, escaped: &str) {
		let mut writer = EscapeComment { inner: String::new() };
		writer.write_str(input).unwrap();
		assert_eq!(writer.inner, escaped);
	}

	check("", "");
	check(" ", " ");
	check("--", "");
	check(" --", " ");
	check("-- ", " ");
}

#[test]
fn test_cdata() {
	#[track_caller]
	fn check(input: &str, escaped: &str) {
		let mut writer = EscapeCharData { inner: String::new() };
		writer.write_str(input).unwrap();
		assert_eq!(writer.inner, escaped);
	}

	check("", "");
	check(" ", " ");
	check("]]>", "]]]]><![CDATA[>");
	check(" ]]>", " ]]]]><![CDATA[>");
	check("]]> ", "]]]]><![CDATA[> ");
}
