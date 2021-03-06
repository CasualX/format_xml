/*!
Replace the standard formatting macros using [template syntax](crate::template!).
*/

/// Replaces `print!` using [template syntax](crate::template!).
#[doc(hidden)]
#[macro_export]
macro_rules! tprint {
	($($tt:tt)+) => { $crate::noinline(|| ::std::print!("{}", $crate::fmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
}
/// Replaces `println!` using [template syntax](crate::template!).
#[doc(hidden)]
#[macro_export]
macro_rules! tprintln {
	($($tt:tt)+) => { $crate::noinline(|| ::std::println!("{}", $crate::fmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
}
/// Replaces `eprint!` using [template syntax](crate::template!).
#[doc(hidden)]
#[macro_export]
macro_rules! teprint {
	($($tt:tt)+) => { $crate::noinline(|| ::std::eprint!("{}", $crate::fmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
}
/// Replaces `eprintln!` using [template syntax](crate::template!).
#[doc(hidden)]
#[macro_export]
macro_rules! teprintln {
	($($tt:tt)+) => { $crate::noinline(|| ::std::eprintln!("{}", $crate::fmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
}
/// Replaces `write!` using [template syntax](crate::template!).
#[doc(hidden)]
#[macro_export]
macro_rules! twrite {
	($dst:expr, $($tt:tt)+) => { $crate::noinline(|| ::core::write!($dst, "{}", $crate::fmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
}
/// Replaces `writeln!` using [template syntax](crate::template!).
#[doc(hidden)]
#[macro_export]
macro_rules! twriteln {
	($dst:expr, $($tt:tt)+) => { $crate::noinline(|| ::core::writeln!($dst, "{}", $crate::fmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
}
/// Replaces `format!` using [template syntax](crate::template!).
#[doc(hidden)]
#[macro_export]
macro_rules! tformat {
	($($tt:tt)+) => { $crate::noinline(|| ::std::format!("{}", $crate::fmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
}
/// Replaces `panic!` using [template syntax](crate::template!).
#[doc(hidden)]
#[macro_export]
macro_rules! tpanic {
	($($tt:tt)+) => { $crate::noinline(|| ::core::panic!("{}", $crate::fmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
}

pub use crate::{
	tprint as print,
	tprintln as println,
	teprint as eprint,
	teprintln as eprintln,
	twrite as write,
	twriteln as writeln,
	tformat as format,
	tpanic as panic,
};

#[test]
fn test_noinline() {
	use std::fmt::Write;
	tprint!("print");
	tprintln!("println");
	teprint!("eprint");
	teprintln!("eprintln");
	let mut s = tformat!("format");
	let _ = twrite!(s, "write");
	let _ = twriteln!(s, "writeln");
	// tpanic!("panic");
}
