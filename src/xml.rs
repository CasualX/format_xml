
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
	(; $fmt:expr, $($args:expr,)*; |$f:ident| { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, "{}"), $($args,)* $crate::FnFmt(|$f| { $($body)* }),; $($tail)*)
	};
	// control
	(; $fmt:expr, $($args:expr,)*; let $p:pat = $e:expr; $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, "{}"), $($args,)* $crate::FnFmt(|f| match $e { $p => f.write_fmt($crate::format_xml!{$($tail)*}) }),;)
	};
	(; $fmt:expr, $($args:expr,)*; if $($tail:tt)*) => {
		$crate::_format_if1_!(; $fmt, $($args,)*; f []; if $($tail)*)
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
// Parse if statements

#[doc(hidden)]
#[macro_export]
macro_rules! _format_if1_ {
	(; $fmt:expr, $($args:expr,)*; $f:ident [$($c:tt)*]; if ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_if2_!(; $fmt, $($args,)*; $f [$($c)* [if ($e) { $f.write_fmt($crate::format_xml!{$($body)*}) }]]; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; $f:ident [$($c:tt)*]; if let $p:pat = ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_if2_!(; $fmt, $($args,)*; $f [$($c)* [if let $p = ($e) { $f.write_fmt($crate::format_xml!{$($body)*}) }]]; $($tail)*)
	};
}
#[doc(hidden)]
#[macro_export]
macro_rules! _format_if2_ {
	(; $fmt:expr, $($args:expr,)*; $f:ident [$($c:tt)*]; else if ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_if2_!(; $fmt, $($args,)*; $f [$($c)* [else if ($e) { $f.write_fmt($crate::format_xml!{$($body)*}) }]]; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; $f:ident [$($c:tt)*]; else if let $p:pat = ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_if2_!(; $fmt, $($args,)*; $f [$($c)* [else if let $p = ($e) { $f.write_fmt($crate::format_xml!{$($body)*}) }]]; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; $f:ident [$([$($c:tt)*])*]; else { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, "{}"), $($args,)* $crate::FnFmt(|$f| $($($c)*)* else { $f.write_fmt($crate::format_xml!{$($body)*}) }),; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; $f:ident [$([$($c:tt)*])*]; $($tail:tt)*) => {
		$crate::_format_tag1_!(; concat!($fmt, "{}"), $($args,)* $crate::FnFmt(|$f| $($($c)*)* else { Ok(()) }),; $($tail)*)
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
	(; $fmt:expr, $($args:expr,)*; {$e:expr} $($tail:tt)*) => {
		$crate::_format_text1_!(; concat!($fmt, "{}"), $($args,)* $e,; $($tail)*)
	};
	(; $fmt:expr, $($args:expr,)*; {$e:expr;$($s:tt)*} $($tail:tt)*) => {
		$crate::_format_text1_!(; concat!($fmt, "{:", $(stringify!($s),)* "}"), $($args,)* $e,; $($tail)*)
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
	($($q:ident!)+; $fmt:expr, $($args:expr,)*; = [$($text:literal : $cond:expr),*$(,)?] $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; concat!($fmt, "=\"{}\""), $($args,)* $crate::FnFmt(|f| { $(if $cond { f.write_str(concat!($text, " "))? })* Ok(()) }),; $($tail)*)
	};
	($($q:ident!)+; $fmt:expr, $($args:expr,)*; = escape!($($body:tt)*) $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; concat!($fmt, "=\"{}\""), $($args,)* $crate::escape!($($body)*),; $($tail)*)
	};
	($($q:ident!)+; $fmt:expr, $($args:expr,)*; = |$f:ident| { $($body:tt)* } $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; concat!($fmt, "=\"{}\""), $($args,)* $crate::FnFmt(|$f| { $($body)* }),; $($tail)*)
	};
	($($q:ident!)+; $fmt:expr, $($args:expr,)*; $($tail:tt)*) => {
		$crate::_format_attrs1_!($($q!)*; $fmt, $($args,)*; $($tail)*)
	};
}
