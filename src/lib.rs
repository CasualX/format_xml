/*!
Fast, minimal, feature-rich, xml-like formatting syntax for Rust!

We say _xml-like_ because due to limitations and flexibility some concessions had to be made.

Features include:

* Arbitrary expressions inside the formatting braces
* Generates optimized Rust code at compiletime
* Auto-escaping control characters.
* Supports rust-analyzer auto complete, refactoring and more!
* Supports Rust's standard formatting specifiers
* Control flow allows conditional and repeated formatting
* Capture variables by value or by reference
* Escape hatch to inject custom formatting code

See [`xfmt!`] for more information.
*/

mod xfmt;
mod prelude;

mod escape;
pub use self::escape::*;

#[doc(hidden)]
pub use fmtools::{__fmt, obfstr};

pub use fmtools::{fmt, join};

#[cfg(doc)]
#[doc = include_str!("../readme.md")]
fn readme() {}
