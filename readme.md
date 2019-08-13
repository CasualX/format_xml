Inline XML Formatting
=====================

Analogue to `format_args` this crate provides a `format_xml` which accepts and formats XML tokens.

In fact `format_xml` is a procedural macro that does nothing more than generate the equivalent `format_args` invocation!

Examples
--------

Let's dive right into a simple example:

```rust
let point = (20, 30);
let name = "World";

let s = format_xml! {
    <svg width=200 height=200>
        <line x1=0 y1=0 x2={point.0} y2={point.1} stroke="black" stroke-width=2 />
        <text x={point.1} y={point.0}>"Hello '" {name} "'!"</text>
    </svg>
}.to_string();
```

The resulting string is `<svg width="200" height="200"><line x1="0" y1="0" x2="20" y2="30" stroke="black" stroke-width="2" /><text x="30" y="20">Hello &apos;World!&apos;</text></svg>`.

Features
--------

Work in Progress
