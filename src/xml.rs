/*!
Replace the standard formatting macros using [xml syntax](macro.xml.html).
*/

/// Replaces `print!` using [xml syntax](macro.format_xml.html).
#[doc(hidden)]
#[macro_export]
macro_rules! xprint {
	($($tt:tt)+) => { $crate::noinline(|| print!("{}", $crate::format_xml!($($tt)*))) };
}
/// Replaces `println!` using [xml syntax](macro.format_xml.html).
#[doc(hidden)]
#[macro_export]
macro_rules! xprintln {
	($($tt:tt)+) => { $crate::noinline(|| println!("{}", $crate::format_xml!($($tt)*))) };
}
/// Replaces `eprint!` using [xml syntax](macro.format_xml.html).
#[doc(hidden)]
#[macro_export]
macro_rules! xeprint {
	($($tt:tt)+) => { $crate::noinline(|| eprint!("{}", $crate::format_xml!($($tt)*))) };
}
/// Replaces `eprintln!` using [xml syntax](macro.format_xml.html).
#[doc(hidden)]
#[macro_export]
macro_rules! xeprintln {
	($($tt:tt)+) => { $crate::noinline(|| eprintln!("{}", $crate::format_xml!($($tt)*))) };
}
/// Replaces `write!` using [xml syntax](macro.format_xml.html).
#[doc(hidden)]
#[macro_export]
macro_rules! xwrite {
	($dst:expr, $($tt:tt)+) => { $crate::noinline(|| write!($dst, "{}", $crate::format_xml!($($tt)*))) };
}
/// Replaces `writeln!` using [xml syntax](macro.format_xml.html).
#[doc(hidden)]
#[macro_export]
macro_rules! xwriteln {
	($dst:expr, $($tt:tt)+) => { $crate::noinline(|| writeln!($dst, "{}", $crate::format_xml!($($tt)*))) };
}
/// Replaces `format!` using [xml syntax](macro.format_xml.html).
#[doc(hidden)]
#[macro_export]
macro_rules! xformat {
	($($tt:tt)+) => { $crate::noinline(|| format!("{}", $crate::format_xml!($($tt)*))) };
}
/// Replaces `panic!` using [xml syntax](macro.format_xml.html).
#[doc(hidden)]
#[macro_export]
macro_rules! xpanic {
	($($tt:tt)+) => { $crate::noinline(|| panic!("{}", $crate::format_xml!($($tt)*))) };
}

pub use crate::{
	xprint as print,
	xprintln as println,
	xeprint as eprint,
	xeprintln as eprintln,
	xwrite as write,
	xwriteln as writeln,
	xformat as format,
	xpanic as panic,
};
