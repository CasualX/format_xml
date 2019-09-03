/*!
Format XML Templating
=====================

Minimal compiletime templating for XML in Rust!

The [`format_xml!` macro](macro.format_xml.html) accepts an XML-like syntax and transforms it into a `format_args!` invocation.
We say _XML-like_ because due to limitations of the macro system some concessions had to be made, see the examples below.

See the [`template!` macro](macro.template.html) to get started with regular formatting.
 */

use std::fmt;

#[macro_use]
mod template;

#[macro_use]
mod xml;

#[macro_use]
mod escape;
pub use self::escape::Escape;

mod html;
pub use self::html::*;

/// Implements `std::fmt::Display` for the Fn closure matching fmt's signature.
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
