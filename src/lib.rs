/*!
Template XML formatting
=======================

Minimal compiletime templating in Rust!

Get started by taking a look at the [`template!`] and [`xml!`] macros.
*/

use std::fmt;

mod template_impl;
pub mod template;

mod xml_impl;
pub mod xml;

// backward compatibility
#[deprecated]
#[doc(hidden)]
pub use crate::xml as format_xml;

mod html;
pub use self::html::*;

#[doc(hidden)]
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct FnFmt<F>(pub F);
impl<F> fmt::Display for FnFmt<F> where F: Fn(&mut fmt::Formatter) -> fmt::Result {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(self.0)(f)
	}
}
impl<F> fmt::Debug for FnFmt<F> where F: Fn(&mut fmt::Formatter) -> fmt::Result {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("fmt([closure])")
	}
}

/// Returns a Display implementation using the provided closure.
#[inline]
pub fn fmt<F>(f: F) -> impl fmt::Display + fmt::Debug where F: Fn(&mut fmt::Formatter) -> fmt::Result {
	FnFmt(f)
}

// Prevent inlining the messy formatting code by moving it into a noinline function.
// This could be done by applying the `#[inline(never)]` directly to the closure but this is experimental:
// error[E0658]: attributes on expressions are experimental
#[doc(hidden)]
#[inline(never)]
pub fn noinline<T, F: FnOnce() -> T>(f: F) -> T { f() }
