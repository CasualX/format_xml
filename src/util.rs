use std::fmt;

/// Formats as String while escaping `&<>"'` with their equivalent entities.
///
/// Accepts either an expression or a list of literals and formatting instructions.
///
/// # Examples
///
/// ```
/// use format_xml::escape;
/// assert_eq!(escape!({"Friday"} "'s the " {13} "th").to_string(), "Friday&apos;s the 13th");
/// assert_eq!(escape!("<script>alert(" {42;#x} ")</script>").to_string(), "&lt;script&gt;alert(0x2a)&lt;/script&gt;");
/// ```
#[macro_export]
macro_rules! escape {
	($e:expr) => {
		$crate::Escape($e)
	};
	({$e:expr}) => {
		$crate::Escape($e)
	};
	($($tt:tt)*) => {
		$crate::_escape_!("",; $($tt)*)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _escape_ {
	($fmt:expr, $($args:expr,)*; ) => {
		$crate::Escape(format_args!($fmt $(,$args)*))
	};
	($fmt:expr, $($args:expr,)*; $text:literal $($tail:tt)*) => {
		$crate::_escape_!(concat!($fmt, $text), $($args,)*; $($tail)*)
	};
	($fmt:expr, $($args:expr,)*; {$e:expr} $($tail:tt)*) => {
		$crate::_escape_!(concat!($fmt, "{}"), $($args,)* $e,; $($tail)*)
	};
	($fmt:expr, $($args:expr,)*; {$e:expr; $($s:tt)*} $($tail:tt)*) => {
		$crate::_escape_!(concat!($fmt, "{:", $(stringify!($s),)* "}"), $($args,)* $e,; $($tail)*)
	};
}

/// Escapes `&<>"'` with their equivalent entities.
///
/// ```
/// use format_xml::Escape;
/// assert_eq!(Escape("&<>\"\'").to_string(), "&amp;&lt;&gt;&quot;&apos;");
/// ```
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct Escape<T>(pub T);
impl<T: fmt::Display> fmt::Display for Escape<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		escape_w(&self.0.to_string(), f)
	}
}
fn escape_w(s: &str, result: &mut dyn fmt::Write) -> fmt::Result {
	for chr in s.chars() {
		match chr {
			'&' => result.write_str("&amp;"),
			'<' => result.write_str("&lt;"),
			'>' => result.write_str("&gt;"),
			'\'' => result.write_str("&apos;"),
			'\"' => result.write_str("&quot;"),
			chr => result.write_char(chr),
		}?;
	}
	Ok(())
}

//----------------------------------------------------------------

/// Formats tuples as a comma separated items.
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct CommaFmt<T>(pub T);
impl<T: fmt::Display, U: fmt::Display> fmt::Display for CommaFmt<(T, U)> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_fmt(format_args!("{},{}", (self.0).0, (self.0).1))
	}
}
impl<T: fmt::Display, U: fmt::Display, V: fmt::Display> fmt::Display for CommaFmt<(T, U, V)> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_fmt(format_args!("{},{},{}", (self.0).0, (self.0).1, (self.0).2))
	}
}
impl<T: fmt::Display> fmt::Display for CommaFmt<[T; 2]> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_fmt(format_args!("{},{}", (self.0)[0], (self.0)[1]))
	}
}
impl<T: fmt::Display> fmt::Display for CommaFmt<[T; 3]> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_fmt(format_args!("{},{},{}", (self.0)[0], (self.0)[1], (self.0)[2]))
	}
}

/// Formats an iterable with spaces between the items.
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct SpacedSet<I>(pub I);
impl<T: fmt::Display, I: Clone + IntoIterator<Item = T>> fmt::Display for SpacedSet<I> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for item in self.0.clone() {
			f.write_fmt(format_args!("{} ", item))?;
		}
		Ok(())
	}
}

/// Formats an iterable with spaces of comma formatted items.
///
/// ```
/// let data = [(1.0, 2.0), (13.0, 42.0), (-5.0, 100.0)];
/// let result = format_xml::format_xml! {
/// 	<polygon points={format_xml::spaced_comma_set(data.iter().cloned())} />
/// }.to_string();
/// assert_eq!(result, r#"<polygon points="1,2 13,42 -5,100 " />"#);
/// ```
pub fn spaced_comma_set<I: IntoIterator>(iterable: I) -> SpacedSet<impl Clone + Iterator<Item = CommaFmt<I::Item>>> where I::IntoIter: Clone {
	SpacedSet(iterable.into_iter().map(CommaFmt))
}
