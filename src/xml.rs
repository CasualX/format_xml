/*!
Replaces the standard formatting macros using [xml syntax](crate::xml!).
*/

/// Replaces `print!` using [xml syntax](crate::xml!).
#[doc(hidden)]
#[macro_export]
macro_rules! xprint {
	($($tt:tt)+) => { $crate::noinline(|| ::std::print!("{}", $crate::xml!($($tt)*))) };
}
/// Replaces `println!` using [xml syntax](crate::xml!).
#[doc(hidden)]
#[macro_export]
macro_rules! xprintln {
	($($tt:tt)+) => { $crate::noinline(|| ::std::println!("{}", $crate::xml!($($tt)*))) };
}
/// Replaces `eprint!` using [xml syntax](crate::xml!).
#[doc(hidden)]
#[macro_export]
macro_rules! xeprint {
	($($tt:tt)+) => { $crate::noinline(|| ::std::eprint!("{}", $crate::xml!($($tt)*))) };
}
/// Replaces `eprintln!` using [xml syntax](crate::xml!).
#[doc(hidden)]
#[macro_export]
macro_rules! xeprintln {
	($($tt:tt)+) => { $crate::noinline(|| ::std::eprintln!("{}", $crate::xml!($($tt)*))) };
}
/// Replaces `write!` using [xml syntax](crate::xml!).
#[doc(hidden)]
#[macro_export]
macro_rules! xwrite {
	($dst:expr, $($tt:tt)+) => { $crate::noinline(|| ::core::write!($dst, "{}", $crate::xml!($($tt)*))) };
}
/// Replaces `writeln!` using [xml syntax](crate::xml!).
#[doc(hidden)]
#[macro_export]
macro_rules! xwriteln {
	($dst:expr, $($tt:tt)+) => { $crate::noinline(|| ::core::writeln!($dst, "{}", $crate::xml!($($tt)*))) };
}
/// Replaces `format!` using [xml syntax](crate::xml!).
#[doc(hidden)]
#[macro_export]
macro_rules! xformat {
	($($tt:tt)+) => { $crate::noinline(|| ::std::format!("{}", $crate::xml!($($tt)*))) };
}
/// Replaces `panic!` using [xml syntax](crate::xml!).
#[doc(hidden)]
#[macro_export]
macro_rules! xpanic {
	($($tt:tt)+) => { $crate::noinline(|| ::core::panic!("{}", $crate::xml!($($tt)*))) };
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

#[test]
fn test_noinline() {
	use std::fmt::Write;
	xprint!(<a>"print"</a>);
	xprintln!(<a>"println"</a>);
	xeprint!(<a>"eprint"</a>);
	xeprintln!(<a>"eprintln"</a>);
	let mut s = xformat!(<a>"format"</a>);
	let _ = xwrite!(s, <a>"write"</a>);
	let _ = xwriteln!(s, <a>"writeln"</a>);
	// xpanic!(<a>"panic"</a>);
}
