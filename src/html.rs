use std::fmt;

/// Formats the arguments while escaping `&<>"'` with their equivalent entities.
///
/// Accepts an expression or a [string template](crate::template!).
///
/// # Examples
///
/// ```
/// # use format_xml::escape;
/// assert_eq!(
/// 	escape!({"Friday"} "'s the " {13} "th").to_string(),
/// 	"Friday&apos;s the 13th");
/// assert_eq!(
/// 	escape!("<script>alert(" {42;#x} ")</script>").to_string(),
/// 	"&lt;script&gt;alert(0x2a)&lt;/script&gt;");
/// ```
#[macro_export]
macro_rules! escape {
	($e:expr) => {
		$crate::escape($e)
	};
	({$e:expr}) => {
		$crate::escape($e)
	};
	($($tt:tt)*) => {
		$crate::escape($crate::fmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))
	};
}

/// Escapes `&<>"'` with their equivalent entities.
///
/// ```
/// # use format_xml::escape;
/// assert_eq!(escape("&<>\"\'").to_string(), "&amp;&lt;&gt;&quot;&apos;");
/// ```
#[inline]
pub fn escape<T: fmt::Display>(value: T) -> impl fmt::Display {
	fn inner(s: &str, f: &mut fmt::Formatter) -> fmt::Result {
		let mut start = 0;
		for i in 0..s.len() {
			let replace = match s.as_bytes()[i] {
				b'&' => "&amp;",
				b'<' => "&lt;",
				b'>' => "&gt;",
				b'\'' => "&apos;",
				b'\"' => "&quot;",
				_ => continue,
			};
			let pre = unsafe { s.get_unchecked(start..i) };
			if pre.len() > 0 {
				f.write_str(pre)?;
			}
			f.write_str(replace)?;
			start = i + 1;
		}
		let post = unsafe { s.get_unchecked(start..) };
		if post.len() > 0 {
			f.write_str(post)?;
		}
		Ok(())
	}
	crate::fmt(move |f| {
		inner(&value.to_string(), f)
	})
}

#[test]
fn test_escape() {
	fn check(input: &str, escaped: &str) {
		assert_eq!(escape(input).to_string(), escaped);
	}
	check("hello", "hello");
	check("'world'", "&apos;world&apos;");
	check("pre<script>post", "pre&lt;script&gt;post");
	check("&<>\"\'", "&amp;&lt;&gt;&quot;&apos;");
	check("&a'b<c>", "&amp;a&apos;b&lt;c&gt;");
}

//----------------------------------------------------------------

/// Displays an iterable with given separator between each item.
///
/// ```
/// # use format_xml::join;
/// let result = join("--", &[1, 2, 3, 4]).to_string();
/// assert_eq!(result, "1--2--3--4");
/// ```
#[inline]
pub fn join<T>(sep: &'static str, collection: T) -> impl fmt::Display where T: IntoIterator, <T as IntoIterator>::Item: fmt::Display, <T as IntoIterator>::IntoIter: Clone {
	let iter = collection.into_iter();
	crate::fmt(move |f| {
		let mut draw = false;
		for item in iter.clone() {
			if sep.len() > 0 {
				if draw {
					f.write_str(sep)?;
				}
				draw = true;
			}
			fmt::Display::fmt(&item, f)?;
		}
		Ok(())
	})
}

/// Displays an iterable with spaces between each item.
///
/// ```
/// # use format_xml::spaced;
/// let result = spaced(&[1, 2, 3, 4]).to_string();
/// assert_eq!(result, "1 2 3 4");
/// ```
#[inline]
pub fn spaced<T>(collection: T) -> impl fmt::Display where T: IntoIterator, <T as IntoIterator>::Item: fmt::Display, <T as IntoIterator>::IntoIter: Clone {
	join(" ", collection)
}

/// Displays an iterable with commas between each item.
///
/// ```
/// # use format_xml::csv;
/// let result = csv(&[1, 2, 3, 4]).to_string();
/// assert_eq!(result, "1,2,3,4");
/// ```
#[inline]
pub fn csv<T>(collection: T) -> impl fmt::Display where T: IntoIterator, <T as IntoIterator>::Item: fmt::Display, <T as IntoIterator>::IntoIter: Clone {
	join(",", collection)
}

//----------------------------------------------------------------

#[doc(hidden)]
#[macro_export]
macro_rules! _join_impl_ {
	(concat!($($fmt:expr,)*), $($arg:expr,)*; $s:literal; $sep:literal, $e:expr, ) => {
		$crate::fmt(move |f| {
			f.write_fmt(format_args!(concat!($($fmt,)* $s), $($arg,)* $e))
		})
	};
	(concat!($($fmt:expr,)*), $($arg:expr,)*; $s:literal; $sep:literal, $e:expr, $($tail:expr,)*) => {
		$crate::_join_impl_!(concat!($($fmt,)* $s, $sep,), $($arg,)* $e,; $s; $sep, $($tail,)*)
	};
}

/// Displays the arguments joined with the given separator.
///
/// The arguments are moved instead of captured by reference unlike `format_args!`.
///
/// ```
/// # use format_xml::join;
/// let result = join!("--", 1, 2.0, true).to_string();
/// assert_eq!(result, "1--2--true");
/// ```
///
/// Optionally, the formatting can be specified:
///
/// ```
/// # use format_xml::join;
/// let result = join!("--", 10, 11, 12; "{:#x}").to_string();
/// assert_eq!(result, "0xa--0xb--0xc");
/// ```
#[macro_export]
macro_rules! join {
	($sep:literal, $($e:expr),+) => {
		$crate::_join_impl_!(concat!(), ; "{}"; $sep, $($e,)+)
	};
	($sep:literal, $($e:expr),+; $s:literal) => {
		$crate::_join_impl_!(concat!(), ; $s; $sep, $($e,)+)
	};
}

/// Displays the arguments as comma-separated values.
///
/// The arguments are moved instead of captured by reference unlike `format_args!`.
///
/// ```
/// # use format_xml::csv;
/// let result = csv!(1, 2.0, true).to_string();
/// assert_eq!(result, "1,2,true");
/// ```
///
/// Optionally, the formatting can be specified:
///
/// ```
/// # use format_xml::csv;
/// let result = csv!(10, 11, 12; "{:#x}").to_string();
/// assert_eq!(result, "0xa,0xb,0xc");
/// ```
#[macro_export]
macro_rules! csv {
	($($e:expr),+) => {
		$crate::_join_impl_!(concat!(), ; "{}"; ",", $($e,)+)
	};
	($($e:expr),+; $s:literal) => {
		$crate::_join_impl_!(concat!(), ; $s; ",", $($e,)+)
	};
}

/// Displays the arguments as space-separated values.
///
/// The arguments are moved instead of captured by reference unlike `format_args!`.
///
/// ```
/// # use format_xml::spaced;
/// let result = spaced!(1, 2.0, true).to_string();
/// assert_eq!(result, "1 2 true");
/// ```
///
/// Optionally, the formatting can be specified:
///
/// ```
/// # use format_xml::spaced;
/// let result = spaced!(10, 11, 12; "{:#x}").to_string();
/// assert_eq!(result, "0xa 0xb 0xc");
/// ```
#[macro_export]
macro_rules! spaced {
	($($e:expr),+) => {
		$crate::_join_impl_!(concat!(), ; "{}"; " ", $($e,)+)
	};
	($($e:expr),+; $s:literal) => {
		$crate::_join_impl_!(concat!(), ; $s; " ", $($e,)+)
	};
}
