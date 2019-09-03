use std::fmt;

/// Formats the arguments while escaping `&<>"'` with their equivalent entities.
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
		$crate::Escape($crate::template!{$($tt)*})
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
fn escape_w<F: fmt::Write>(s: &str, result: &mut F) -> fmt::Result {
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
