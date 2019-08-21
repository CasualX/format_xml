/*!
Format XML Templating
=====================

Minimal compile time templating for XML in Rust!

The `format_xml!` macro by example accepts an XML-like syntax and transforms it into a `format_args!` invocation.
We say _XML-like_ because due to limitations of the macro system some concessions had to be made, see the examples below.

Examples
--------

### Basic usage

```rust
# use format_xml::format_xml;
let point = (20, 30);
let name = "World";

# let result =
format_xml! {
	<svg width="200" height="200">
		<line x1="0" y1="0" x2={point.0} y2={point.1} stroke="black" stroke-width="2" />
		<text x={point.1} y={point.0}>"Hello '" {name} "'!"</text>
	</svg>
}.to_string()
# ; assert_eq!(result, r#"<svg width="200" height="200"><line x1="0" y1="0" x2="20" y2="30" stroke="black" stroke-width="2" /><text x="30" y="20">Hello 'World'!</text></svg>"#);
```

The resulting string is `<svg width="200" height="200"><line x1="0" y1="0" x2="20" y2="30" stroke="black" stroke-width="2" /><text x="30" y="20">Hello 'World!'</text></svg>`.

Note how the expression values to be formatted are inlined in the formatting braces.

### Formatting specifiers

```rust
# use format_xml::format_xml;
let value = 42;

# let result =
format_xml! {
	<span data-value={value}>{value;#x?}</span>
}.to_string()
# ; assert_eq!(result, r#"<span data-value="42">0x2a</span>"#);
```

The resulting string is `<span data-value="42">0x2a</span>`.

Due to limitations of macros by example, a semicolon is used to separate the value from the formatting specifiers. The rules for the specifiers are exactly the same as the standard library of Rust.

### Supported tags

```rust
# use format_xml::format_xml;
# let result =
format_xml! {
	<!doctype html>
	<?xml version="1.0" encoding="UTF-8"?>
	<tag-name></tag-name>
	<ns:self-closing-tag />
	<!-- "comment" -->
	<![CDATA["cdata"]]>
}.to_string()
# ; assert_eq!(result, r#"<!doctype html><?xml version="1.0" encoding="UTF-8"?><tag-name></tag-name><ns:self-closing-tag /><!-- comment --><![CDATA[cdata]]>"#);
```

The resulting string is `<!doctype html><?xml version="1.0" encoding="UTF-8"?><open-tag></open-tag><ns:self-closing-tag />`.

### Control flow

```rust
# use format_xml::format_xml;
let switch = true;
let opt = Some("World");
let result: Result<f32, i32> = Err(13);

# let result =
format_xml! {
	if let Some(name) = (opt) {
		<h1>"Hello " {name}</h1>
	}
	if (switch) {
		match (result) {
			Ok(f) => { <i>{f}</i> }
			Err(i) => { <b>{i}</b> }
		}
		<ul>
		for i in (1..=5) {
			let times_five = i * 5;
			<li>{i}"*5="{times_five}</li>
		}
		</ul>
	}
}.to_string()
# ; assert_eq!(result, r#"<h1>Hello World</h1><b>13</b><ul><li>1*5=5</li><li>2*5=10</li><li>3*5=15</li><li>4*5=20</li><li>5*5=25</li></ul>"#);
```

The resulting string is `<h1>Hello World</h1><ul><li>1*5=5</li><li>2*5=10</li><li>3*5=15</li><li>4*5=20</li><li>5*5=25</li></ul>`.

Control flow are currently only supported outside tags. They are not supported in attributes. The expressions for `if` and `for` must be surrounded with parentheses due to macro by example limitations.

Limitations
-----------

This crate is implemented with standard macros by example (`macro_rules!`). Because of this there are various limitations:

* It is not possible to check whether tags are closed by the appropriate closing tag. This crate will happily accept `<open></close>`. It does enforce more simple lexical rules such as rejecting `</tag/>`.

* Escaping of `&<>"'` is not automatic. You can trivially break the structure by including these characters in either the formatting string or formatted values. Avoid untrusted input!

* The formatting specifiers are separated from its value by a semicolon instead of a colon.

* The compiler may complain about macro expansion recursion limit being reached, simply apply the suggested fix and increase the limit. This crate implements a 'tt muncher' which are known to hit these limits.

* Text nodes must be valid Rust literals. Bare words are not supported.

 */

use std::fmt;

mod util;
pub use self::util::*;

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

/// Format XML tokens with `format_args!`
///
/// See the [module-level documentation](index.html) for more information.
#[macro_export]
macro_rules! format_xml {
	($($tt:tt)*) => {
		$crate::_format_tag1_!(; "",; $($tt)*)
	};
}

//----------------------------------------------------------------
// Parse xml tags, text nodes and control flow

#[doc(hidden)]
#[macro_export]
macro_rules! _format_tag1_ {
	// tags
	(; $fmt:expr, $($args:expr,)*; </ $($tail:tt)*) => {
		$crate::_format_ident1_!(_format_tag3_!; concat!($fmt, "</"), $($args,)*; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; <? $($tail:tt)*) => {
		$crate::_format_ident1_!(_format_attrs1_! _format_tag4_!; concat!($fmt, "<?"), $($args,)*; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; <!-- $($tail:tt)*) => {
		$crate::_format_text1_!(; concat!($fmt, "<!-- "), $($args,)*; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; <![CDATA[ $($text:literal)* ]]> $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, "<![CDATA[", $($text,)* "]]>"), $($args,)*; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; <! $($tail:tt)*) => {
		$crate::_format_ident1_!(_format_attrs1_! _format_tag2_!; concat!($fmt, "<!"), $($args,)*; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; < $($tail:tt)*) => {
		$crate::_format_ident1_!(_format_attrs1_! _format_tag2_!; concat!($fmt, "<"), $($args,)*; $($tail)*)
	};
	// text
	(; $fmt:expr, $($args:expr,)*; $text:literal $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, $text), $($args,)*; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; {$e:expr} $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, "{}"), $($args,)* $e,; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; {$e:expr;$($s:tt)*} $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, "{:", $(stringify!($s),)* "}"), $($args,)* $e,; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; escape!($($body:tt)*) $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, "{}"), $($args,)* $crate::escape!($($body)*),; $($tail)*)
	};
	// control
	(; $fmt:expr, $($args:expr,)*; let $p:pat = $e:expr; $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, "{}"), $($args,)* $crate::FnFmt(|f| match $e { $p => f.write_fmt($crate::format_xml!{$($tail)*}) }),;)
	};
	(; $fmt:expr, $($args:expr,)*; if ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, "{}"), $($args,)* $crate::FnFmt(|f| if $e { f.write_fmt($crate::format_xml!{$($body)*}) } else { Ok(()) }),; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; if let $p:pat = ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, "{}"), $($args,)* $crate::FnFmt(|f| if let $p = $e { f.write_fmt($crate::format_xml!{$($body)*}) } else { Ok(()) }),; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; match ($e:expr) { $($p:pat => { $($body:tt)* })* } $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, "{}"), $($args,)* $crate::FnFmt(|f| match $e { $($p => f.write_fmt($crate::format_xml!{$($body)*}),)* }),; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; for $p:pat in ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, "{}"), $($args,)* $crate::FnFmt(|f| { for $p in $e { f.write_fmt($crate::format_xml!{$($body)*})?; } Ok(()) }),; $($tail)*)
	};
	// term
	(; $fmt:expr, $($args:expr,)*;) => {
		format_args!($fmt $(,$args)*)
	};
}
#[doc(hidden)]
#[macro_export]
macro_rules! _format_tag2_ {
	(; $fmt:expr, $($args:expr,)*; > $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, ">"), $($args,)*; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; /> $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, " />"), $($args,)*; $($tail)*)
	};
}
#[doc(hidden)]
#[macro_export]
macro_rules! _format_tag3_ {
	(; $fmt:expr, $($args:expr,)*; > $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, ">"), $($args,)*; $($tail)*)
	};
}
#[doc(hidden)]
#[macro_export]
macro_rules! _format_tag4_ {
	(; $fmt:expr, $($args:expr,)*; ?> $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, "?>"), $($args,)*; $($tail)*)
	};
}

//----------------------------------------------------------------
// Parse comments and CDATA

#[doc(hidden)]
#[macro_export]
macro_rules! _format_text1_ {
	(; $fmt:expr, $($args:expr,)*; --> $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, " -->"), $($args,)*; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; $text:literal $($tail:tt)*) => {
		$crate::_format_text1_!(; concat!($fmt, $text), $($args,)*; $($tail)*)
	};
}

//----------------------------------------------------------------
// Parse xml names

#[doc(hidden)]
#[macro_export]
macro_rules! _format_ident1_ {
	($($q:ident!)+; $fmt:expr, $($args:expr,)*; $name:ident : $($tail:tt)*) => {
		$crate::_format_ident2_!($($q!)+; concat!($fmt, stringify!($name), ":"), $($args,)*; $($tail)*)
	};
	($($q:ident!)+; $fmt:expr, $($args:expr,)*; $name:ident - $($tail:tt)*) => {
		$crate::_format_ident2_!($($q!)+; concat!($fmt, stringify!($name), "-"), $($args,)*; $($tail)*)
	};
	($next:ident! $($q:ident!)*; $fmt:expr, $($args:expr,)*; $name:ident $($tail:tt)*) => {
		$crate::$next!($($q!)*; concat!($fmt, stringify!($name)), $($args,)*; $($tail)*)
	};
}
#[doc(hidden)]
#[macro_export]
macro_rules! _format_ident2_ {
	($($q:ident!)+; $fmt:expr, $($args:expr,)*; $name:ident - $($tail:tt)*) => {
		$crate::_format_ident2_!($($q!)+; concat!($fmt, stringify!($name), "-"), $($args,)*; $($tail)*)
	};
	($next:ident! $($q:ident!)*; $fmt:expr, $($args:expr,)*; $name:ident $($tail:tt)*) => {
		$crate::$next!($($q!)*; concat!($fmt, stringify!($name)), $($args,)*; $($tail)*)
	};
}

//----------------------------------------------------------------
// Parse xml attributes

#[doc(hidden)]
#[macro_export]
macro_rules! _format_attrs1_ {
	($($q:ident!)+; $fmt:expr, $($args:expr,)*; $tail_id:ident $($tail:tt)*) => {
		$crate::_format_ident1_!(_format_attrs2_! $($q!)+; concat!($fmt, " "), $($args,)*; $tail_id $($tail)*)
	};
	($next:ident! $($q:ident!)*; $fmt:expr, $($args:expr,)*; $($tail:tt)*) => {
		$crate::$next!($($q!)*; $fmt, $($args,)*; $($tail)*)
	};
}
#[doc(hidden)]
#[macro_export]
macro_rules! _format_attrs2_ {
	($($q:ident!)+; $fmt:expr, $($args:expr,)*; = $text:literal $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; concat!($fmt, "=\"", $text, '"'), $($args,)*; $($tail)*)
	};
	($($q:ident!)+; $fmt:expr, $($args:expr,)*; = {$e:expr} $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; concat!($fmt, "=\"{}\""), $($args,)* $e,; $($tail)*)
	};
	($($q:ident!)+; $fmt:expr, $($args:expr,)*; = {$e:expr; $($s:tt)*} $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; concat!($fmt, "=\"{:", $(stringify!($s),)* "}\""), $($args,)* $e,; $($tail)*)
	};
	($($q:ident!)+; $fmt:expr, $($args:expr,)*; = escape!($($body:tt)*) $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; concat!($fmt, "=\"{}\""), $($args,)* $crate::escape!($($body)*),; $($tail)*)
	};
	($($q:ident!)+; $fmt:expr, $($args:expr,)*; $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; $fmt, $($args,)*; $($tail)*)
	};
}
