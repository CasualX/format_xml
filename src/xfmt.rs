/// Xml-like formatting syntax.
///
/// Returns a displayable object which can be formatted with `{}`.
///
/// # Examples
///
/// ### Basic usage
///
/// ```rust
/// let point = (20, 30);
/// let name = "World";
///
/// # let result =
/// format_xml::xfmt! {
/// 	<svg width="200" height="200">
/// 		<line x1="0" y1="0" x2={point.0} y2={point.1} stroke="black" stroke-width="2" />
/// 		<text x={point.1} y={point.0}>"Hello '"{name}"'!"</text>
/// 	</svg>
/// }
/// # .to_string();
/// # assert_eq!(result, r#"<svg width="200" height="200"><line x1="0" y1="0" x2="20" y2="30" stroke="black" stroke-width="2" /><text x="30" y="20">Hello 'World'!</text></svg>"#);
/// ```
///
/// The resulting string is `<svg width="200" height="200"><line x1="0" y1="0" x2="20" y2="30" stroke="black" stroke-width="2" /><text x="30" y="20">Hello 'World!'</text></svg>`.
///
/// The value arguments can be arbitrary expressions. They are inlined in the formatting braces and are outside the string literals.
///
/// The values inside formatting braces are escaped by default, the text literals are not.
/// Use the [escape hatch](#escape-hatch) to bypass automatic escaping.
///
/// ### Formatting specifiers
///
/// ```rust
/// let value = 42;
///
/// # let result =
/// format_xml::xfmt! {
/// 	<span data-value={value}>{value:#x?}</span>
/// }
/// # .to_string();
/// # assert_eq!(result, r#"<span data-value="42">0x2a</span>"#);
/// ```
///
/// The resulting string is `<span data-value="42">0x2a</span>`.
///
/// The rules for the specifiers are exactly the same as Rust's [standard formatting syntax](std::fmt).
///
/// ### Escaping
///
/// ```rust
/// let value = "\"quote\"";
/// let text = "<script>&</script>";
/// # let result =
/// format_xml::xfmt! {
/// 	<p data-value={value}>{text}</p>
/// }
/// # .to_string();
/// # assert_eq!(result, r#"<p data-value="&quot;quote&quot;">&lt;script&gt;&amp;&lt;/script&gt;</p>"#);
/// ```
///
/// The resulting string is `<p data-value="&quot;quote&quot;">&lt;script&gt;&amp;&lt;/script&gt;</p>`.
///
/// The values inside formatting braces are escaped by default, the text literals are not.
///
/// * Text elements escape `<`, `&`, `>`.
/// * Attribute values escape `<`, `&`, `>`, `'`, `"`.
/// * Comment nodes escape `--` by removing it altogether.
/// * CDATA sections escape `]]>`.
///
/// Escaping is not implemented in some HTML contexts:
/// inside `<script>`, `<style>` tags or their respective attribute equivalents (event handlers and inline styles),
/// do not format user controlled values in these locations!
///
/// ### Supported syntax
///
/// ```rust
/// # let result =
/// format_xml::xfmt! {
/// 	<!doctype html>
/// 	<?xml version="1.0" encoding="UTF-8"?>
/// 	<tag-name></tag-name>
/// 	<self-closing-tag />
/// 	<!-- "comment" -->
/// 	<![CDATA["cdata"]]>
/// }
/// # .to_string();
/// # assert_eq!(result, r#"<!doctype html><?xml version="1.0" encoding="UTF-8"?><tag-name></tag-name><self-closing-tag /><!-- comment --><![CDATA[cdata]]>"#);
/// ```
///
/// The resulting string is `<!doctype html><?xml version="1.0" encoding="UTF-8"?><tag-name></tag-name><self-closing-tag /><!-- comment --><![CDATA[cdata]]>`.
///
/// Examples of element naming and namespace syntax support:
///
/// ```rust
/// # let result =
/// format_xml::xfmt! {
/// 	<tag>
/// 	<tag-foo>
/// 	<tag.foo>
/// 	<ns:tag>
/// 	<"_t-0.z">
/// }
/// # .to_string();
/// # assert_eq!(result, r#"<tag><tag-foo><tag.foo><ns:tag><_t-0.z>"#);
/// ```
///
/// The resulting string is `<tag><tag-foo><tag.foo><ns:tag><_t-0.z>`.
///
/// There are no restrictions on matching open/close tags or reject tags which cannot be self-closing.
///
/// Unfinished implementation:
///
/// * Document type definitions (DTD) are not correctly implemented. The `<!doctype>` tag is barely functional.
/// * Processing instructions are not correctly implemented. The `<?xml?>` tag is barely functional.
///
/// ### Control flow
///
/// ```rust
/// let switch = true;
/// let opt = Some("World");
///
/// # let result =
/// format_xml::xfmt! {
/// 	if let Some(name) = (opt) {
/// 		<h1>"Hello "{name}</h1>
/// 	}
/// 	else if (switch) {
/// 		<h1>"Hello User"</h1>
/// 	}
/// }
/// # .to_string();
/// # assert_eq!(result, "<h1>Hello World</h1>");
/// ```
///
/// The resulting string is `<h1>Hello World</h1>`.
///
/// ```rust
/// let result: Result<f32, i32> = Err(13);
///
/// # let result =
/// format_xml::xfmt! {
/// 	match result {
/// 		Ok(f) => <i>{f}</i>,
/// 		Err(i) => <b>{i}</b>,
/// 	}
/// }
/// # .to_string();
/// # assert_eq!(result, "<b>13</b>");
/// ```
///
/// The resulting string is `<b>13</b>`.
///
/// ```rust
/// # let result =
/// format_xml::xfmt! {
/// 	<ul>
/// 	for i in (1..=5) {
/// 		let times_five = i * 5;
/// 		<li>{i}"*5="{times_five}</li>
/// 	}
/// 	</ul>
/// }
/// # .to_string();
/// # assert_eq!(result, "<ul><li>1*5=5</li><li>2*5=10</li><li>3*5=15</li><li>4*5=20</li><li>5*5=25</li></ul>");
/// ```
///
/// The resulting string is `<ul><li>1*5=5</li><li>2*5=10</li><li>3*5=15</li><li>4*5=20</li><li>5*5=25</li></ul>`.
///
/// Control flow is only supported outside tags, not in attributes.
///
/// ### Escape hatch
///
/// ```rust
/// fn compose(f: &mut std::fmt::Formatter, a: i32) -> std::fmt::Result {
/// 	format_xml::write!(f, <span>{a}</span>)
/// }
///
/// # let result =
/// format_xml::xfmt! {
/// 	<p>|f| compose(f, 42)?;</p>
/// }
/// # .to_string();
/// # assert_eq!(result, r#"<p><span>42</span></p>"#);
/// ```
///
/// The resulting string is `<p><span>42</span></p>`.
///
/// Closure syntax provides an escape hatch to inject code if needed.
/// The argument's type is [`&mut Formatter`](std::fmt::Formatter).
///
/// Important! Anything written to the formatter `f` is not escaped.
/// This makes it useful to compose different components wich is not possible with `{}`.
#[macro_export]
macro_rules! xfmt {
	(move $($tt:tt)*) => {
		$crate::fmt(move |_f| {
			$crate::__xfmt!{_f concat() $($tt)*}
			Ok(())
		})
	};
	($($tt:tt)*) => {
		$crate::fmt(|_f| {
			$crate::__xfmt!{_f concat() $($tt)*}
			Ok(())
		})
	};
}



#[macro_export]
#[doc(hidden)]
macro_rules! __xfmt {
	// tag close
	($f:ident concat($($texts:expr,)*) </ @ident($tag:expr) > $($tail:tt)*) => {
		$crate::__xfmt!{$f concat($($texts,)* "</", $tag, ">",) $($tail)*}
	};
	($f:ident concat($($texts:expr,)*) </ $($tail:tt)*) => {
		$crate::__xfmt_ident!{__xfmt! [$f concat($($texts,)*) </] $($tail)*}
	};

	// comment
	($f:ident concat($($texts:expr,)*) <!-- $($tail:tt)*) => {
		$crate::__write_str!{$f concat($($texts,)* "<!-- ",)}
		$crate::__xfmt_comment!{$f () $($tail)*}
	};

	// CDATA
	($f:ident concat($($texts:expr,)*) <![CDATA[ $($tt:tt)* ]]> $($tail:tt)*) => {
		$crate::__write_str!{$f concat($($texts,)* "<![CDATA[",)}
		{
			let _f = $crate::EscapeCharData::wrap($f);
			$crate::__fmt!{_f $($tt)*}
		}
		$crate::__xfmt!{$f concat("]]>",) $($tail)*}
	};

	// doctype
	($f:ident concat($($texts:expr,)*) <! @ident($tag:expr) $($tail:tt)*) => {
		$crate::__xfmt_attrs!{__xfmt_close_tag! $f concat($($texts,)* "<!", $tag,) $($tail)*}
	};
	($f:ident concat($($texts:expr,)*) <! $($tail:tt)*) => {
		$crate::__xfmt_ident!{__xfmt! [$f concat($($texts,)*) <!] $($tail)*}
	};

	// declaration
	($f:ident concat($($texts:expr,)*) <? @ident($tag:expr) $($tail:tt)*) => {
		$crate::__xfmt_attrs!{__xfmt_close_decl! $f concat($($texts,)* "<?", $tag,) $($tail)*}
	};
	($f:ident concat($($texts:expr,)*) <? $($tail:tt)*) => {
		$crate::__xfmt_ident!{__xfmt! [$f concat($($texts,)*) <?] $($tail)*}
	};

	// tag open
	($f:ident concat($($texts:expr,)*) < @ident($tag:expr) $($tail:tt)*) => {
		$crate::__xfmt_attrs!{__xfmt_close_tag! $f concat($($texts,)* "<", $tag,) $($tail)*}
	};
	($f:ident concat($($texts:expr,)*) < $($tail:tt)*) => {
		$crate::__xfmt_ident!{__xfmt! [$f concat($($texts,)*) <] $($tail)*}
	};

	// text
	($f:ident concat($($texts:expr,)*) $text1:literal $text2:literal $($tail:tt)*) => {
		$crate::__xfmt!{$f concat($($texts,)* $text1, $text2,) $($tail)*}
	};
	($f:ident concat($($texts:expr,)*) $text:literal $($tail:tt)*) => {
		$crate::__xfmt!{$f concat($($texts,)* $text,) $($tail)*}
	};

	// format
	($f:ident concat($($texts:expr,)*) {$($e:tt)*} $($tail:tt)*) => {
		$crate::__write_str!{$f concat($($texts,)*)}
		::core::fmt::write($crate::EscapeText::wrap($f), $crate::__xfmt_format!([] $($e)*))?;
		$crate::__xfmt!{$f concat() $($tail)*}
	};

	// escape hatch
	($f:ident concat($($texts:expr,)*) |$ff:pat_param| $block:block $($tail:tt)*) => {
		$crate::__write_str!{$f concat($($texts,)*)}
		let $ff = &mut *$f;
		$block
		$crate::__xfmt!{$f concat() $($tail)*}
	};
	($f:ident concat($($texts:expr,)*) |$ff:pat_param| $stmt:stmt; $($tail:tt)*) => {
		$crate::__write_str!{$f concat($($texts,)*)}
		let $ff = &mut *$f;
		$stmt
		$crate::__xfmt!{$f concat() $($tail)*}
	};

	// let
	($f:ident concat($($texts:expr,)*) let $p:pat = $e:expr; $($tail:tt)*) => {
		let $p = $e;
		$crate::__xfmt!{$f concat($($texts,)*) $($tail)*}
	};

	// if
	($f:ident concat($($texts:expr,)*) if $($tail:tt)*) => {
		$crate::__write_str!{$f concat($($texts,)*)}
		$crate::__xfmt_if!{$f [] if $($tail)*}
	};

	// match
	($f:ident concat($($texts:expr,)*) match ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::__write_str!{$f concat($($texts,)*)}
		$crate::__xfmt_match!{$f match ($e) {} $($body)*}
		$crate::__xfmt!{$f concat() $($tail)*}
	};
	($f:ident concat($($texts:expr,)*) match $($tail:tt)*) => {
		$crate::__with_parens!{__xfmt! [$f concat($($texts:expr,)*) match] [] $($tail)*}
	};

	// for
	($f:ident concat($($texts:expr,)*) for $p:pat in ($e:expr) { $($body:tt)*} $($tail:tt)*) => {
		$crate::__write_str!{$f concat($($texts,)*)}
		for $p in $e {
			$crate::__xfmt!{$f concat() $($body)*}
		}
		$crate::__xfmt!{$f concat() $($tail)*}
	};
	($f:ident concat($($texts:expr,)*) for $p:pat in $($tail:tt)*) => {
		$crate::__with_parens!{__xfmt! [$f concat($($texts,)*) for $p in] [] $($tail)*}
	};

	// optimization
	($f:ident concat($($texts:expr,)*) ($($tt:tt)*) $($tail:tt)*) => {
		$crate::__xfmt!{$f concat($($texts,)*) $($tt)*}
		$crate::__xfmt!{$f concat() $($tail)*}
	};

	// term
	($f:ident concat()) => {};
	($f:ident concat($($texts:expr,)*)) => {
		$crate::__write_str!{$f concat($($texts,)*)}
	};
}



#[macro_export]
#[doc(hidden)]
macro_rules! __xfmt_close_tag {
	($f:ident concat($($texts:expr,)*) > $($tail:tt)*) => {
		$crate::__xfmt!{$f concat($($texts,)* ">",) $($tail)*}
	};
	($f:ident concat($($texts:expr,)*) /> $($tail:tt)*) => {
		$crate::__xfmt!{$f concat($($texts,)* " />",) $($tail)*}
	};
	($f:ident concat($($texts:expr,)*)) => {
		$crate::__xfmt!{$f concat($($texts,)* ">",)}
		compile_error!("missing closing `>`");
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __xfmt_close_decl {
	($f:ident concat($($texts:expr,)*) ?> $($tail:tt)*) => {
		$crate::__xfmt!{$f concat($($texts,)* "?>",) $($tail)*}
	};
	($f:ident concat($($texts:expr,)*)) => {
		$crate::__xfmt!{$f concat($($texts,)* "?>",)}
		compile_error!("missing closing `?>`");
	};
}



#[macro_export]
#[doc(hidden)]
macro_rules! __xfmt_attrs {
	($term:ident! $f:ident concat($($texts:expr,)*) @ident($key:expr) = $($tail:tt)*) => {
		$crate::__xfmt_attrvalue!{$term! $f concat($($texts,)* " ", $key, "=",) $($tail)*}
	};
	($term:ident! $f:ident concat($($texts:expr,)*) @ident($key:expr) $($tail:tt)*) => {
		$crate::__xfmt_attrs!{$term! $f concat($($texts,)* " ", $key,) $($tail)*}
	};
	($term:ident! $f:ident concat($($texts:expr,)*) $key:ident $($tail:tt)*) => {
		$crate::__xfmt_ident!{__xfmt_attrs! [$term! $f concat($($texts,)*) ] $key $($tail)*}
	};
	($term:ident! $f:ident concat($($texts:expr,)*) $key:literal = $($tail:tt)*) => {
		$crate::__xfmt_attrvalue!{$term! $f concat($($texts,)* " ", $key, "=",) $($tail)*}
	};
	($term:ident! $f:ident concat($($texts:expr,)*) $($tail:tt)*) => {
		$crate::$term!{$f concat($($texts,)*) $($tail)*}
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __xfmt_attrvalue {
	($term:ident! $f:ident concat($($texts:expr,)*) $text:literal $($tail:tt)*) => {
		$crate::__xfmt_attrs!{$term! $f concat($($texts,)* "\"", $text, "\"",) $($tail)*}
	};
	($term:ident! $f:ident concat($($texts:expr,)*) {$($e:tt)*} $($tail:tt)*) => {
		$crate::__write_str!{$f concat($($texts,)* "\"",)}
		::core::fmt::write($crate::EscapeAttrValue::wrap($f), $crate::__xfmt_format!([] $($e)*))?;
		$crate::__xfmt_attrs!{$term! $f concat("\"",) $($tail)*}
	};
	($term:ident! $f:ident concat($($texts:expr,)*) |$ff:pat_param| $block:block $($tail:tt)*) => {
		$crate::__write_str!{$f concat($($texts,)* "\"",)}
		let $ff = &mut *$f;
		$block
		$crate::__xfmt_attrs!{$term! $f concat("\"",) $($tail)*}
	};
	($term:ident! $f:ident concat($($texts:expr,)*) |$ff:pat_param| $stmt:stmt; $($tail:tt)*) => {
		$crate::__write_str!{$f concat($($texts,)* "\"",)}
		let $ff = &mut *$f;
		$stmt
		$crate::__xfmt_attrs!{$term! $f concat("\"",) $($tail)*}
	};
}


// Parse an xml identifier:
//
// * A single (string) literal to bypass any of the other rules
// * Start with an optional namespace identifier followed by ':'
// * An identifier followed by either '-' or '.' and more
// * A final identifier
//
// Wrap the whole identifier in a `@ident(*)`
// Where * is the concat'd and stringified identifier.
#[macro_export]
#[doc(hidden)]
macro_rules! __xfmt_ident {
	($next:ident! [$($prefix:tt)*] $name:literal $($tail:tt)*) => {
		$crate::$next!{$($prefix)* @ident(concat!($name)) $($tail)*}
	};
	($next:ident! [$($prefix:tt)*] $frag:ident: $($tail:tt)*) => {
		$crate::__xfmt_ident_cont!{$next! [$($prefix)*] [stringify!($frag), ":",] $($tail)*}
	};
	($next:ident! [$($prefix:tt)*] $frag:ident- $($tail:tt)*) => {
		$crate::__xfmt_ident_cont!{$next! [$($prefix)*] [stringify!($frag), "-",] $($tail)*}
	};
	($next:ident! [$($prefix:tt)*] $frag:ident. $($tail:tt)*) => {
		$crate::__xfmt_ident_cont!{$next! [$($prefix)*] [stringify!($frag), ".",] $($tail)*}
	};
	($next:ident! [$($prefix:tt)*] $frag:ident $($tail:tt)*) => {
		$crate::$next!{$($prefix)* @ident(stringify!($frag)) $($tail)*}
	};
}

#[macro_export]
#[doc(hidden)]
macro_rules! __xfmt_ident_cont {
	($next:ident! [$($prefix:tt)*] [$($tt:expr,)*] $frag:ident- $($tail:tt)*) => {
		$crate::__xfmt_ident_cont!{$next! [$($prefix)*] [$($tt,)* stringify!($frag), "-",] $($tail)*}
	};
	($next:ident! [$($prefix:tt)*] [$($tt:expr,)*] $frag:ident. $($tail:tt)*) => {
		$crate::__xfmt_ident_cont!{$next! [$($prefix)*] [$($tt,)* stringify!($frag), ".",] $($tail)*}
	};
	($next:ident! [$($prefix:tt)*] [$($tt:expr,)*] $frag:ident $($tail:tt)*) => {
		$crate::$next!{$($prefix)* @ident(concat!($($tt,)* stringify!($frag))) $($tail)*}
	};
}



#[macro_export]
#[doc(hidden)]
macro_rules! __xfmt_comment {
	($f:ident ($($tt:tt)*) --> $($tail:tt)*) => {
		{
			let _f = $crate::EscapeComment::wrap($f);
			$crate::__fmt!{_f $($tt)*}
		}
		$crate::__xfmt!{$f concat(" -->",) $($tail)*}
	};
	($f:ident ($($tt:tt)*) $nom:tt $($tail:tt)*) => {
		$crate::__xfmt_comment!{$f ($($tt)* $nom) $($tail)*}
	};
}



#[doc(hidden)]
#[macro_export]
macro_rules! __xfmt_format {
	([$($e:tt)*] : $($tail:tt)*) => {
		$crate::__xfmt_format_expr!([$($e)*] : $($tail)*)
	};
	([$($e:tt)*] ; $($tail:tt)*) => {
		$crate::__xfmt_format_expr!([$($e)*] : $($tail)*)
	};
	([$($e:tt)*] $nom:tt $($tail:tt)*) => {
		$crate::__xfmt_format!([$($e)* $nom] $($tail)*)
	};
	([$($e:tt)*]) => {
		$crate::__xfmt_format_expr!([$($e)*])
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __xfmt_format_expr {
	([$e:expr]) => {
		::core::format_args!("{}", $e)
	};
	([$e:expr $(, $w:expr)?] $($s:tt)*) => {
		::core::format_args!(concat!("{", $(stringify!($s),)* "}"), $e $(,$w)?)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __write_str {
	($f:ident concat()) => {};
	($f:ident concat($($texts:expr,)+)) => {
		$f.write_str($crate::obfstr!(concat!($($texts),+)))?;
	};
}



#[doc(hidden)]
#[macro_export]
macro_rules! __xfmt_if {
	// if let
	($f:ident [$($c:tt)*] if let $p:pat = ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::__xfmt_if!{$f [$($c)* if let $p = $e { $crate::__xfmt!{$f concat() $($body)*} }] $($tail)*}
	};
	($f:ident [$($c:tt)*] if let $p:pat = $($tail:tt)*) => {
		$crate::__with_parens!{__xfmt_if! [$f [$($c)*] if let $p =] [] $($tail)*}
	};

	// if
	($f:ident [$($c:tt)*] if ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::__xfmt_if!{$f [$($c)* if $e { $crate::__xfmt!{$f concat() $($body)*} }] $($tail)*}
	};
	($f:ident [$($c:tt)*] if $($tail:tt)*) => {
		$crate::__with_parens!{__xfmt_if! [$f [$($c)*] if] [] $($tail)*}
	};

	// else if let
	($f:ident [$($c:tt)*] else if let $p:pat = ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::__xfmt_if!{$f [$($c)* else if let $p = $e { $crate::__xfmt!{$f concat() $($body)*} }] $($tail)*}
	};
	($f:ident [$($c:tt)*] else if let $p:pat = $($tail:tt)*) => {
		$crate::__with_parens!{__xfmt_if! [$f [$($c)*] else if let $p =] [] $($tail)*}
	};

	// else if
	($f:ident [$($c:tt)*] else if ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::__xfmt_if!{$f [$($c)* else if $e { $crate::__xfmt!{$f concat() $($body)*} }] $($tail)*}
	};
	($f:ident [$($c:tt)*] else if $($tail:tt)*) => {
		$crate::__with_parens!{__xfmt_if! [$f [$($c)*] else if] [] $($tail)*}
	};

	// else
	($f:ident [$($c:tt)*] else { $($body:tt)* } $($tail:tt)*) => {
		$($c)*
		else {
			$crate::__xfmt!{$f concat() $($body)*}
		}
		$crate::__xfmt!{$f concat() $($tail)*}
	};

	// term
	($f:ident [$($c:tt)*] $($tail:tt)*) => {
		$($c)*
		$crate::__xfmt!{$f concat() $($tail)*}
	};
}



#[doc(hidden)]
#[macro_export]
macro_rules! __xfmt_match {
	($f:ident match ($e:expr) {$($arms:tt)*} $p:pat $(if $guard:expr)? => { $($body:tt)* }, $($tail:tt)*) => {
		$crate::__xfmt_match!{$f match ($e) {$($arms)* $p $(if $guard)? => { $crate::__xfmt!{$f concat() $($body)*} }} $($tail)*}
	};
	($f:ident match ($e:expr) {$($arms:tt)*} $p:pat $(if $guard:expr)? => { $($body:tt)* } $($tail:tt)*) => {
		$crate::__xfmt_match!{$f match ($e) {$($arms)* $p $(if $guard)? => { $crate::__xfmt!{$f concat() $($body)*} }} $($tail)*}
	};
	($f:ident match ($e:expr) {$($arms:tt)*} $p:pat $(if $guard:expr)? => $($tail:tt)*) => {
		$crate::__until_comma!{__xfmt_match! [$f match ($e) {$($arms)*} $p $(if $guard)? =>] {} $($tail)*}
	};
	($f:ident match ($e:expr) {$($pat:pat $(if $guard:expr)? => $block:block)*}) => {
		match $e {
			$($pat $(if $guard)? => $block)*
		}
	};
}



#[doc(hidden)]
#[macro_export]
macro_rules! __with_parens {
	($next:ident! [$($prefix:tt)*] [$($tt:tt)*] { $($body:tt)* } $($tail:tt)*) => {
		$crate::$next!{$($prefix)* ($($tt)*) { $($body)* } $($tail)*}
	};
	($next:ident! [$($prefix:tt)*] [$($tt:tt)*] $nom:tt $($tail:tt)*) => {
		$crate::__with_parens!{$next! [$($prefix)*] [$($tt)* $nom] $($tail)*}
	};
	// Allows auto complete to work without the following {}
	($next:ident! [$($prefix:tt)*] [$($tt:tt)*]) => {
		$crate::$next!{$($prefix)* ($($tt)*) {}}
		compile_error!(concat!("missing block after expression: ", stringify!($($tt)*)));
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __until_comma {
	($next:ident! [$($prefix:tt)*] {$($tt:tt)*} , $($tail:tt)*) => {
		$crate::$next!{$($prefix)* {$($tt)*}, $($tail)*}
	};
	($next:ident! [$($prefix:tt)*] {$($tt:tt)*} $nom:tt $($tail:tt)*) => {
		$crate::__until_comma!{$next! [$($prefix)*] {$($tt)* $nom} $($tail)*}
	};
	($next:ident! [$($prefix:tt)*] {$($tt:tt)*}) => {
		$crate::$next!{$($prefix)* {$($tt)*}}
	};
}



#[test]
fn test_syntax() {
	let c = 5i32;

	let _ = xfmt! {
		(<a> <a-b> <"a-0"> <a.b> <"a.0"> <"0">)
		(<xmlns:a> <xmlns:a-b> <xmlns:a.b>)
		(</a> </a-b> </"a-0"> </a.b> </"a.0"> </"0">)
		(<tag a="hello" a-b='a' "a-0"=0 a.b="a.b">)
		<self-closing />
		<?open?>"{}"
		<!-- --><!-- "asd" --><!-- 0 '1' "2" 0.3 -->
		<![CDATA[if c.is_positive() { "Hello world!" }]]>
		{42}
	};
	let _ = xfmt!{if true {}};
	let _ = xfmt!{for _ in 0..4 {}};
	let _ = xfmt!{match true { false => "false", true => "true"}};
}
