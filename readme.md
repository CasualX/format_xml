Xml-like formatting
===================

[![MIT License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/format_xml.svg)](https://crates.io/crates/format_xml)
[![docs.rs](https://docs.rs/format_xml/badge.svg)](https://docs.rs/format_xml)
[![Build status](https://github.com/CasualX/format_xml/workflows/CI/badge.svg)](https://github.com/CasualX/format_xml/actions)

Fast, minimal, feature-rich, xml-like formatting syntax for Rust!

We say _xml-like_ because due to limitations and flexibility some concessions had to be made; see the examples below.

Features include:

* Arbitrary expressions inside the formatting braces
* Generates optimized Rust code at compiletime
* Supports rust-analyzer auto complete, refactoring and more!
* Supports Rust's standard formatting specifiers
* Control flow allows conditional and repeated formatting
* Capture variables by value or by reference
* Escape hatch to inject custom formatting code

In your Cargo.toml, add:

```text
[dependencies]
format_xml = "0.2"
```

Examples
--------

### Basic usage

```rust
let point = (20, 30);
let name = "World";

let string = format_xml::format! {
	<svg width="200" height="200">
		<line x1="0" y1="0" x2={point.0} y2={point.1} stroke="black" stroke-width="2" />
		<text x={point.1} y={point.0}>"Hello '"{name}"'!"</text>
	</svg>
};

assert_eq!(string, r#"<svg width="200" height="200"><line x1="0" y1="0" x2="20" y2="30" stroke="black" stroke-width="2" /><text x="30" y="20">Hello 'World'!</text></svg>"#);
```

The value arguments can be arbitrary expressions. They are inlined in the formatting braces and are outside the string literals.

The values inside formatting braces are escaped by default, the text literals are not.
Use the [escape hatch](#escape-hatch) to bypass the automatic escaping.

### Formatting specifiers

```rust
let value = 42;

let string = format_xml::format! {
	<span data-value={value}>{value:#x?}</span>
};

assert_eq!(string, r#"<span data-value="42">0x2a</span>"#);
```

The rules for the specifiers are exactly the same as [the standard library](https://doc.rust-lang.org/std/fmt/index.html) of Rust.

### Escaping

```rust
let value = "\"quote\"";
let text = "<script>&</script>";

let string = format_xml::format! {
	<p data-value={value}>{text}</p>
};

assert_eq!(string, r#"<p data-value="&quot;quote&quot;">&lt;script&gt;&amp;&lt;/script&gt;</p>"#);
```

The values inside formatting braces are escaped by default, the text literals are not.

* Text elements escape `<`, `&`, `>`.
* Attribute values escape `<`, `&`, `>`, `'`, `"`.
* Comment nodes escape `--` by removing it altogether.
* CDATA sections escape `]]>`.

Escaping is not implemented in some HTML contexts:
inside `<script>`, `<style>` tags or their respective attribute equivalents (event lers and inline styles),
do not format user controlled values in these locations!

### Supported syntax

```rust
let string = format_xml::format! {
	<!doctype html>
	<?xml version="1.0" encoding="UTF-8"?>
	<tag-name></tag-name>
	<self-closing-tag />
	<!-- "comment" -->
	<![CDATA["cdata"]]>
};

assert_eq!(string, r#"<!doctype html><?xml version="1.0" encoding="UTF-8"?><tag-name></tag-name><self-closing-tag /><!-- comment --><![CDATA[cdata]]>"#);
```

Examples of element naming and namespace syntax support:

```rust
let string = format_xml::format! {
	<tag>
	<tag-foo>
	<tag.foo>
	<ns:tag>
	<"_t-0.z">
};

assert_eq!(string, r#"<tag><tag-foo><tag.foo><ns:tag><_t-0.z>"#);
```

There are no restrictions on matching open/close tags or reject tags which cannot be self-closing.

Unfinished implementation:

* Document type definitions (DTD) are not correctly implemented. The `<!doctype>` tag is barely functional.
* Processing instructions are not correctly implemented. The `<?xml?>` tag is barely functional.

### Control flow

```rust
let switch = true;
let opt = Some("World");

let string = format_xml::format! {
	if let Some(name) = (opt) {
		<h1>"Hello "{name}</h1>
	}
	else if (switch) {
		<h1>"Hello User"</h1>
	}
};

assert_eq!(string, "<h1>Hello World</h1>");
```

```rust
let string: Result<f32, i32> = Err(13);

let string = format_xml::format! {
	match string {
		Ok(f) => <i>{f}</i>,
		Err(i) => <b>{i}</b>,
	}
};

assert_eq!(string, "<b>13</b>");
```

```rust
let string = format_xml::format! {
	<ul>
	for i in (1..=5) {
		let times_five = i * 5;
		<li>{i}"*5="{times_five}</li>
	}
	</ul>
};

assert_eq!(string, "<ul><li>1*5=5</li><li>2*5=10</li><li>3*5=15</li><li>4*5=20</li><li>5*5=25</li></ul>");
```

Control flow is only supported outside tags, not in attributes.

### Escape hatch

```rust
fn compose(f: &mut std::fmt::Formatter, a: i32) -> std::fmt::Result {
	format_xml::write!(f, <span>{a}</span>)
}

let string = format_xml::format! {
	<p>|f| compose(f, 42)?;</p>
};

assert_eq!(string, r#"<p><span>42</span></p>"#);
```

Closure syntax provides an escape hatch to inject code if needed.
The argument's type is [`&mut Formatter`](https://doc.rust-lang.org/std/fmt/struct.Formatter.html).

Important! Anything written to the formatter `f` is not escaped.
This makes it useful to compose different components wich is not possible with `{}`.

License
-------

Licensed under [MIT License](https://opensource.org/licenses/MIT), see [license.txt](license.txt).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, shall be licensed as above, without any additional terms or conditions.
