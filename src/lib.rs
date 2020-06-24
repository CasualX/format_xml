/*!
Template XML formatting
=======================

Minimal compiletime templating in Rust!

Get started by taking a look at the [`template!`](macro.template.html) and [`xml!`](macro.xml.html) macros.
*/

use std::fmt;

mod template_impl;
pub mod template;

mod xml_impl;
pub mod xml;

// backward compatibility
#[doc(hidden)]
pub use crate::xml as format_xml;

mod escape;
pub use self::escape::Escape;

mod html;
pub use self::html::*;

/// Implements Display for closures.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct FnFmt<F: Fn(&mut fmt::Formatter) -> fmt::Result>(pub F);
impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for FnFmt<F> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(self.0)(f)
	}
}
impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Debug for FnFmt<F> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("FnFmt([closure])")
	}
}

// Prevent inlining the messy formatting code by moving it into a noinline function.
// This could be done by applying the `#[inline(never)]` directly to the closure but this is experimental:
// error[E0658]: attributes on expressions are experimental
#[doc(hidden)]
#[inline(never)]
pub fn noinline<T, F: FnOnce() -> T>(f: F) -> T { f() }
