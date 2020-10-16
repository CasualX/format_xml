Template XML formatting
=======================

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/format_xml.svg)](https://crates.io/crates/format_xml)
[![docs.rs](https://docs.rs/format_xml/badge.svg)](https://docs.rs/format_xml)

Minimal compiletime templating for XML in Rust!

The `xml!` macro accepts an XML-like syntax and transforms it into a `format_args!` invocation. We say _XML-like_ because due to limitations of declarative macros some concessions had to be made; see the examples below.

Features of this crate include providing the value to be formatted inline in the formatting braces and control flow for conditionally formatting all in one simple package with zero dependencies!

In your Cargo.toml, add:

```
[dependencies]
format_xml = "0.1"
```

Examples
--------

### Basic usage

```rust
let point = (20, 30);
let name = "World";

xml! {
	<svg width="200" height="200">
		<line x1="0" y1="0" x2={point.0} y2={point.1} stroke="black" stroke-width="2" />
		<text x={point.1} y={point.0}>"Hello '" {name} "'!"</text>
	</svg>
}.to_string()
```

The resulting string is `<svg width="200" height="200"><line x1="0" y1="0" x2="20" y2="30" stroke="black" stroke-width="2" /><text x="30" y="20">Hello 'World'!</text></svg>`.

Note how the expression values to be formatted are inlined in the formatting braces.

### Formatting specifiers

```rust
let value = 42;

xml! {
	<span data-value={value}>{value;#x?}</span>
}.to_string()
```

The resulting string is `<span data-value="42">0x2a</span>`.

Due to limitations of declarative macros, a semicolon is used to separate the value from the formatting specifiers. The rules for the specifiers are exactly the same as [the standard library](https://doc.rust-lang.org/std/fmt/index.html) of Rust.

### Composition

```rust
fn compose(f: &mut std::fmt::Formatter, a: i32) -> std::fmt::Result {
	f.write_fmt(xml! {
		<span>{a}</span>
	})
}

xml! {
	<p>|f| { compose(f, 42) }</p>
}.to_string()
```

The resulting string is `<p><span>42</span></p>`.

Closure syntax allows capturing of the underlying formatter like you would when writing a custom fmt trait. This enables to compose the final XML from reusable subtemplates.

### Supported tags

```rust
xml! {
	<!doctype html>
	<?xml version="1.0" encoding="UTF-8"?>
	<tag-name></tag-name>
	<ns:self-closing-tag />
	<!-- "comment" -->
	<![CDATA["cdata"]]>
}.to_string()
```

The resulting string is `<!doctype html><?xml version="1.0" encoding="UTF-8"?><tag-name></tag-name><ns:self-closing-tag /><!-- comment --><![CDATA[cdata]]>`.

### Control flow

```rust
let switch = true;
let opt = Some("World");
let result: Result<f32, i32> = Err(13);

xml! {
	if let Some(name) = (opt) {
		<h1>"Hello " {name}</h1>
	}
	else if (switch) {
		<h1>"Hello User"</h1>
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
	else {
		<p>"No contents"</p>
	}
}.to_string()
```

The resulting string is `<h1>Hello World</h1><b>13</b><ul><li>1*5=5</li><li>2*5=10</li><li>3*5=15</li><li>4*5=20</li><li>5*5=25</li></ul>`.

Control flow are currently only supported outside tags. They are not supported in attributes. The expressions for `if` and `for` must be surrounded with parentheses due to declarative macro limitations.

### Specialised attribute syntax

```rust
let has_a = true;
let has_b = false;
let make_red = true;

xml! {
	<div class=["class-a": has_a, "class-b": has_b]>
		<span style=["color: red;": make_red]></span>
		<p data-attr=("has_a:"{has_a}",has_b:"{has_b})></p>
		<p data-fmt=|f| { f.write_str(if make_red { "MAKE_RED" } else { "" }) }></p>
	</div>
}.to_string()
```

The resulting string is `<div class="class-a "><span style="color: red; "></span><p data-attr="has_a:true,has_b:false"></p><p data-fmt="MAKE_RED"></p></div>`.

Dedicated syntax for fixed set of space delimited attribute values where each element can be conditionally included. This is specifically designed to work with the style and class attributes of html.

If attributes require more advanced formatting, the `template!` syntax is available by wrapping the value in parentheses.
For even more power closure syntax is available to write custom formatting code. The curly braces are required.

Limitations
-----------

This crate is implemented with declarative macros. Because of this, there are various limitations:

* It is not possible to check whether tags are closed by the appropriate closing tag. This crate will happily accept `<open></close>`. It does enforce more simple lexical rules such as rejecting `</tag/>`.

* Escaping of `<`, `&`, `>` and `"` is not automatic. You can trivially break the structure by including these characters in either the formatting string or formatted values. Avoid untrusted input!

* The formatting specifiers are separated from its value by a semicolon instead of a colon.

* The compiler may complain about macro expansion recursion limit being reached, simply apply the suggested fix and increase the limit. This crate implements a 'tt muncher' which are known to hit these limits.

* Text nodes must be valid Rust literals. Bare words are not supported.

* Braces must be escaped, eg. `"{{ }}"` results in a single set of `{ }`.

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
