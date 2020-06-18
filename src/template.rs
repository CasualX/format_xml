/*!
Replace the standard formatting macros using [template syntax](macro.template.html).
*/

/// Replaces `print!` using [template syntax](../macro.template.html).
#[doc(hidden)]
#[macro_export]
macro_rules! tprint {
	($($tt:tt)+) => { $crate::noinline(|| print!("{}", $crate::FnFmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
}
/// Replaces `println!` using [template syntax](../macro.template.html).
#[doc(hidden)]
#[macro_export]
macro_rules! tprintln {
	($($tt:tt)+) => { $crate::noinline(|| println!("{}", $crate::FnFmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
}
/// Replaces `eprint!` using [template syntax](../macro.template.html).
#[doc(hidden)]
#[macro_export]
macro_rules! teprint {
	($($tt:tt)+) => { $crate::noinline(|| eprint!("{}", $crate::FnFmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
}
/// Replaces `eprintln!` using [template syntax](../macro.template.html).
#[doc(hidden)]
#[macro_export]
macro_rules! teprintln {
	($($tt:tt)+) => { $crate::noinline(|| eprintln!("{}", $crate::FnFmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
}
/// Replaces `twrite!` using [template syntax](../macro.template.html).
#[doc(hidden)]
#[macro_export]
macro_rules! twrite {
	($dst:expr, $($tt:tt)+) => { $crate::noinline(|| write!($dst, "{}", $crate::FnFmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
}
/// Replaces `twriteln!` using [template syntax](../macro.template.html).
#[doc(hidden)]
#[macro_export]
macro_rules! twriteln {
	($dst:expr, $($tt:tt)+) => { $crate::noinline(|| writeln!($dst, "{}", $crate::FnFmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
}
/// Replaces `format!` using [template syntax](../macro.template.html).
#[doc(hidden)]
#[macro_export]
macro_rules! tformat {
	($($tt:tt)+) => { $crate::noinline(|| format!("{}", $crate::FnFmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
}
/// Replaces `panic!` using [template syntax](../macro.template.html).
#[doc(hidden)]
#[macro_export]
macro_rules! tpanic {
	($($tt:tt)+) => { $crate::noinline(|| panic!("{}", $crate::FnFmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) }))) };
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
