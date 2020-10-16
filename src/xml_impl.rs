/// Template XML-like.
///
/// Compiles down to a single `format_args!` invocation.
///
/// # Safety
///
/// Arguments are not escaped by default!
/// Use the [`escape!`](macro.escape.html) macro to format and html escape its contents.
///
/// # Examples
///
/// ### Basic usage
///
/// ```rust
/// # use format_xml::xml;
/// #
/// let point = (20, 30);
/// let name = "World";
///
/// # let result =
/// xml! {
/// 	<svg width="200" height="200">
/// 		<line x1="0" y1="0" x2={point.0} y2={point.1} stroke="black" stroke-width="2" />
/// 		<text x={point.1} y={point.0}>"Hello '" {name} "'!"</text>
/// 	</svg>
/// }.to_string()
/// # ; assert_eq!(result, r#"<svg width="200" height="200"><line x1="0" y1="0" x2="20" y2="30" stroke="black" stroke-width="2" /><text x="30" y="20">Hello 'World'!</text></svg>"#);
/// ```
///
/// The resulting string is `<svg width="200" height="200"><line x1="0" y1="0" x2="20" y2="30" stroke="black" stroke-width="2" /><text x="30" y="20">Hello 'World!'</text></svg>`.
///
/// Note how the expression values to be formatted are inlined in the formatting braces.
///
/// ### Formatting specifiers
///
/// ```rust
/// # use format_xml::xml;
/// #
/// let value = 42;
///
/// # let result =
/// xml! {
/// 	<span data-value={value}>{value;#x?}</span>
/// }.to_string()
/// # ; assert_eq!(result, r#"<span data-value="42">0x2a</span>"#);
/// ```
///
/// The resulting string is `<span data-value="42">0x2a</span>`.
///
/// Due to limitations of declarative macros, a semicolon is used to separate the value from the formatting specifiers. The rules for the specifiers are exactly the same as [the standard library](https://doc.rust-lang.org/std/fmt/index.html) of Rust.
///
/// ### Composition
///
/// ```rust
/// # use format_xml::xml;
/// #
/// fn compose(f: &mut std::fmt::Formatter, a: i32) -> std::fmt::Result {
/// 	f.write_fmt(xml! {
/// 		<span>{a}</span>
/// 	})
/// }
///
/// # let result =
/// xml! {
/// 	<p>|f| { compose(f, 42) }</p>
/// }.to_string()
/// # ; assert_eq!(result, r#"<p><span>42</span></p>"#);
/// ```
///
/// The resulting string is `<p><span>42</span></p>`.
///
/// Closure syntax allows capturing of the underlying formatter like you would when writing a custom fmt trait. This enables to compose the final XML from reusable subtemplates.
///
/// ### Supported tags
///
/// ```rust
/// # use format_xml::xml;
/// #
/// # let result =
/// xml! {
/// 	<!doctype html>
/// 	<?xml version="1.0" encoding="UTF-8"?>
/// 	<tag-name></tag-name>
/// 	<ns:self-closing-tag />
/// 	<!-- "comment" -->
/// 	<![CDATA["cdata"]]>
/// }.to_string()
/// # ; assert_eq!(result, r#"<!doctype html><?xml version="1.0" encoding="UTF-8"?><tag-name></tag-name><ns:self-closing-tag /><!-- comment --><![CDATA[cdata]]>"#);
/// ```
///
/// The resulting string is `<!doctype html><?xml version="1.0" encoding="UTF-8"?><tag-name></tag-name><ns:self-closing-tag /><!-- comment --><![CDATA[cdata]]>`.
///
/// ### Control flow
///
/// ```rust
/// # use format_xml::xml;
/// #
/// let switch = true;
/// let opt = Some("World");
/// let result: Result<f32, i32> = Err(13);
///
/// # let result =
/// xml! {
/// 	if let Some(name) = (opt) {
/// 		<h1>"Hello " {name}</h1>
/// 	}
/// 	else if (switch) {
/// 		<h1>"Hello User"</h1>
/// 	}
/// 	if (switch) {
/// 		match (result) {
/// 			Ok(f) => { <i>{f}</i> }
/// 			Err(i) => { <b>{i}</b> }
/// 		}
/// 		<ul>
/// 		for i in (1..=5) {
/// 			let times_five = i * 5;
/// 			<li>{i}"*5="{times_five}</li>
/// 		}
/// 		</ul>
/// 	}
/// 	else {
/// 		<p>"No contents"</p>
/// 	}
/// }.to_string()
/// # ; assert_eq!(result, r#"<h1>Hello World</h1><b>13</b><ul><li>1*5=5</li><li>2*5=10</li><li>3*5=15</li><li>4*5=20</li><li>5*5=25</li></ul>"#);
/// ```
///
/// The resulting string is `<h1>Hello World</h1><b>13</b><ul><li>1*5=5</li><li>2*5=10</li><li>3*5=15</li><li>4*5=20</li><li>5*5=25</li></ul>`.
///
/// Control flow are currently only supported outside tags. They are not supported in attributes. The expressions for `if` and `for` must be surrounded with parentheses due to declarative macro limitations.
///
/// ### Specialised attribute syntax
///
/// ```rust
/// # use format_xml::xml;
/// #
/// let has_a = true;
/// let has_b = false;
/// let make_red = true;
///
/// # let result =
/// xml! {
/// 	<div class=["class-a": has_a, "class-b": has_b]>
/// 		<span style=["color: red;": make_red]></span>
/// 		<p data-attr=("has_a:"{has_a}",has_b:"{has_b})></p>
/// 		<p data-fmt=|f| { f.write_str(if make_red { "MAKE_RED" } else { "" }) }></p>
/// 	</div>
/// }.to_string()
/// # ; assert_eq!(result, r#"<div class="class-a "><span style="color: red; "></span><p data-attr="has_a:true,has_b:false"></p><p data-fmt="MAKE_RED"></p></div>"#);
/// ```
///
/// The resulting string is `<div class="class-a "><span style="color: red; "></span><p data-attr="has_a:true,has_b:false"></p><p data-fmt="MAKE_RED"></p></div>`.
///
/// Dedicated syntax for fixed set of space delimited attribute values where each element can be conditionally included. This is specifically designed to work with the style and class attributes of html.
///
/// If attributes require more advanced formatting, the `template!` syntax is available by wrapping the value in parentheses.
/// For even more power closure syntax is available to write custom formatting code. The curly braces are required.
///
/// Limitations
/// -----------
///
/// This crate is implemented with declarative macros. Because of this, there are various limitations:
///
/// * It is not possible to check whether tags are closed by the appropriate closing tag. This crate will happily accept `<open></close>`. It does enforce more simple lexical rules such as rejecting `</tag/>`.
///
/// * Escaping of `&<>"'` is not automatic. You can trivially break the structure by including these characters in either the formatting string or formatted values. Avoid untrusted input!
///
/// * The formatting specifiers are separated from its value by a semicolon instead of a colon.
///
/// * The compiler may complain about macro expansion recursion limit being reached, simply apply the suggested fix and increase the limit. This crate implements a 'tt muncher' which are known to hit these limits.
///
/// * Text nodes must be valid Rust literals. Bare words are not supported.
///
/// * Braces must be escaped, eg. `"{{ }}"` results in a single set of `{ }`.
#[macro_export]
macro_rules! xml {
	($($tt:tt)*) => {
		$crate::_format_tag1_!(; concat!(),; $($tt)*)
	};
}

//----------------------------------------------------------------
// Parse xml tags, text nodes and control flow

#[doc(hidden)]
#[macro_export]
macro_rules! _format_tag1_ {
	// tags
	(; concat!($($fmt:expr,)*), $($args:expr,)*; </ $($tail:tt)*) => {
		$crate::_format_ident1_!(_format_tag3_!; concat!($($fmt,)* "</",), $($args,)*; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; <? $($tail:tt)*) => {
		$crate::_format_ident1_!(_format_attrs1_! _format_tag4_!; concat!($($fmt,)* "<?",), $($args,)*; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; <!-- $($tail:tt)*) => {
		$crate::_format_text1_!(; concat!($($fmt,)* "<!-- ",), $($args,)*; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; <![CDATA[ $($text:literal)* ]]> $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* "<![CDATA[", $($text,)* "]]>",), $($args,)*; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; <! $($tail:tt)*) => {
		$crate::_format_ident1_!(_format_attrs1_! _format_tag2_!; concat!($($fmt,)* "<!",), $($args,)*; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; < $($tail:tt)*) => {
		$crate::_format_ident1_!(_format_attrs1_! _format_tag2_!; concat!($($fmt,)* "<",), $($args,)*; $($tail)*)
	};
	// text
	(; concat!($($fmt:expr,)*), $($args:expr,)*; $text:literal $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* $text,), $($args,)*; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; {$e:expr} $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* "{}",), $($args,)* $e,; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; {$e:expr;$($s:tt)*} $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* "{:", $(stringify!($s),)* "}",), $($args,)* $e,; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; escape!($($body:tt)*) $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* "{}",), $($args,)* $crate::escape!($($body)*),; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; |$f:ident| { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* "{}",), $($args,)* $crate::FnFmt(|$f| { $($body)* }),; $($tail)*)
	};
	// control
	(; concat!($($fmt:expr,)*), $($args:expr,)*; let $p:pat = $e:expr; $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* "{}",), $($args,)* $crate::FnFmt(|f| match $e { $p => f.write_fmt($crate::xml!{$($tail)*}) }),;)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; if $($tail:tt)*) => {
		$crate::_format_if1_!(; concat!($($fmt,)*), $($args,)*; f []; if $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; match ($e:expr) { $($p:pat => { $($body:tt)* })* } $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* "{}",), $($args,)* $crate::FnFmt(|f| match $e { $($p => f.write_fmt($crate::xml!{$($body)*}),)* }),; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; for $p:pat in ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* "{}",), $($args,)* $crate::FnFmt(|f| { for $p in $e { f.write_fmt($crate::xml!{$($body)*})?; } Ok(()) }),; $($tail)*)
	};
	// term
	(; concat!($($fmt:expr,)*), $($args:expr,)*;) => {
		::core::format_args!(concat!($($fmt,)*) $(,$args)*)
	};
}
#[doc(hidden)]
#[macro_export]
macro_rules! _format_tag2_ {
	(; concat!($($fmt:expr,)*), $($args:expr,)*; > $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* ">",), $($args,)*; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; /> $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* " />",), $($args,)*; $($tail)*)
	};
}
#[doc(hidden)]
#[macro_export]
macro_rules! _format_tag3_ {
	(; concat!($($fmt:expr,)*), $($args:expr,)*; > $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* ">",), $($args,)*; $($tail)*)
	};
}
#[doc(hidden)]
#[macro_export]
macro_rules! _format_tag4_ {
	(; concat!($($fmt:expr,)*), $($args:expr,)*; ?> $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* "?>",), $($args,)*; $($tail)*)
	};
}

//----------------------------------------------------------------
// Parse if statements

#[doc(hidden)]
#[macro_export]
macro_rules! _format_if1_ {
	(; concat!($($fmt:expr,)*), $($args:expr,)*; $f:ident [$($c:tt)*]; if ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_if2_!(; concat!($($fmt,)*), $($args,)*; $f [$($c)* [if ($e) { $f.write_fmt($crate::xml!{$($body)*}) }]]; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; $f:ident [$($c:tt)*]; if let $p:pat = ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_if2_!(; concat!($($fmt,)*), $($args,)*; $f [$($c)* [if let $p = ($e) { $f.write_fmt($crate::xml!{$($body)*}) }]]; $($tail)*)
	};
}
#[doc(hidden)]
#[macro_export]
macro_rules! _format_if2_ {
	(; concat!($($fmt:expr,)*), $($args:expr,)*; $f:ident [$($c:tt)*]; else if ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_if2_!(; concat!($($fmt,)*), $($args,)*; $f [$($c)* [else if ($e) { $f.write_fmt($crate::xml!{$($body)*}) }]]; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; $f:ident [$($c:tt)*]; else if let $p:pat = ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_if2_!(; concat!($($fmt,)*), $($args,)*; $f [$($c)* [else if let $p = ($e) { $f.write_fmt($crate::xml!{$($body)*}) }]]; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; $f:ident [$([$($c:tt)*])*]; else { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* "{}",), $($args,)* $crate::FnFmt(|$f| $($($c)*)* else { $f.write_fmt($crate::xml!{$($body)*}) }),; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; $f:ident [$([$($c:tt)*])*]; $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* "{}",), $($args,)* $crate::FnFmt(|$f| $($($c)*)* else { Ok(()) }),; $($tail)*)
	};
}

//----------------------------------------------------------------
// Parse comments and CDATA

#[doc(hidden)]
#[macro_export]
macro_rules! _format_text1_ {
	(; concat!($($fmt:expr,)*), $($args:expr,)*; --> $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($($fmt,)* " -->",), $($args,)*; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; $text:literal $($tail:tt)*) => {
		$crate::_format_text1_!(; concat!($($fmt,)* $text,), $($args,)*; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; {$e:expr} $($tail:tt)*) => {
		$crate::_format_text1_!(; concat!($($fmt,)* "{}",), $($args,)* $e,; $($tail)*)
	};
	(; concat!($($fmt:expr,)*), $($args:expr,)*; {$e:expr;$($s:tt)*} $($tail:tt)*) => {
		$crate::_format_text1_!(; concat!($($fmt,)* "{:", $(stringify!($s),)* "}",), $($args,)* $e,; $($tail)*)
	};
}

//----------------------------------------------------------------
// Parse xml names

#[doc(hidden)]
#[macro_export]
macro_rules! _format_ident1_ {
	($($q:ident!)+; concat!($($fmt:expr,)*), $($args:expr,)*; $name:ident : $($tail:tt)*) => {
		$crate::_format_ident2_!($($q!)+; concat!($($fmt,)* stringify!($name), ":",), $($args,)*; $($tail)*)
	};
	($($q:ident!)+; concat!($($fmt:expr,)*), $($args:expr,)*; $name:ident - $($tail:tt)*) => {
		$crate::_format_ident2_!($($q!)+; concat!($($fmt,)* stringify!($name), "-",), $($args,)*; $($tail)*)
	};
	($next:ident! $($q:ident!)*; concat!($($fmt:expr,)*), $($args:expr,)*; $name:ident $($tail:tt)*) => {
		$crate::$next!($($q!)*; concat!($($fmt,)* stringify!($name),), $($args,)*; $($tail)*)
	};
}
#[doc(hidden)]
#[macro_export]
macro_rules! _format_ident2_ {
	($($q:ident!)+; concat!($($fmt:expr,)*), $($args:expr,)*; $name:ident - $($tail:tt)*) => {
		$crate::_format_ident2_!($($q!)+; concat!($($fmt,)* stringify!($name), "-",), $($args,)*; $($tail)*)
	};
	($next:ident! $($q:ident!)*; concat!($($fmt:expr,)*), $($args:expr,)*; $name:ident $($tail:tt)*) => {
		$crate::$next!($($q!)*; concat!($($fmt,)* stringify!($name),), $($args,)*; $($tail)*)
	};
}

//----------------------------------------------------------------
// Parse xml attributes

#[doc(hidden)]
#[macro_export]
macro_rules! _format_attrs1_ {
	($($q:ident!)+; concat!($($fmt:expr,)*), $($args:expr,)*; $tail_id:ident $($tail:tt)*) => {
		$crate::_format_ident1_!(_format_attrs2_! $($q!)+; concat!($($fmt,)* " ",), $($args,)*; $tail_id $($tail)*)
	};
	($next:ident! $($q:ident!)*; concat!($($fmt:expr,)*), $($args:expr,)*; $($tail:tt)*) => {
		$crate::$next!($($q!)*; concat!($($fmt,)*), $($args,)*; $($tail)*)
	};
}
#[doc(hidden)]
#[macro_export]
macro_rules! _format_attrs2_ {
	($($q:ident!)+; concat!($($fmt:expr,)*), $($args:expr,)*; = $text:literal $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; concat!($($fmt,)* "=\"", $text, '"',), $($args,)*; $($tail)*)
	};
	($($q:ident!)+; concat!($($fmt:expr,)*), $($args:expr,)*; = {$e:expr} $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; concat!($($fmt,)* "=\"{}\"",), $($args,)* $e,; $($tail)*)
	};
	($($q:ident!)+; concat!($($fmt:expr,)*), $($args:expr,)*; = {$e:expr; $($s:tt)*} $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; concat!($($fmt,)* "=\"{:", $(stringify!($s),)* "}\"",), $($args,)* $e,; $($tail)*)
	};
	($($q:ident!)+; concat!($($fmt:expr,)*), $($args:expr,)*; = [$($text:literal : $cond:expr),*$(,)?] $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; concat!($($fmt,)* "=\"{}\"",), $($args,)* $crate::FnFmt(|f| { $(if $cond { f.write_str(concat!($text, " "))? })* Ok(()) }),; $($tail)*)
	};
	($($q:ident!)+; concat!($($fmt:expr,)*), $($args:expr,)*; = ($($body:tt)*) $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; concat!($($fmt,)* "=\"{}\"",), $($args,)* $crate::FnFmt(|f| f.write_fmt($crate::template!($($body)*))),; $($tail)*)
	};
	($($q:ident!)+; concat!($($fmt:expr,)*), $($args:expr,)*; = escape!($($body:tt)*) $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; concat!($($fmt,)* "=\"{}\"",), $($args,)* $crate::escape!($($body)*),; $($tail)*)
	};
	($($q:ident!)+; concat!($($fmt:expr,)*), $($args:expr,)*; = |$f:ident| { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; concat!($($fmt,)* "=\"{}\"",), $($args,)* $crate::FnFmt(|$f| { $($body)* }),; $($tail)*)
	};
	($($q:ident!)+; concat!($($fmt:expr,)*), $($args:expr,)*; $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; concat!($($fmt,)*), $($args,)*; $($tail)*)
	};
}
