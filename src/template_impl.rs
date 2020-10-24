/// Template strings.
///
/// Compiles down to a single `format_args!` invocation.
///
/// # Examples
///
/// ### Basic usage
///
/// ```
/// # use format_xml::template;
/// let name = "World";
///
/// # let s =
/// template!("Hello "{name}"!").to_string()
/// # ;assert_eq!(s, "Hello World!");
/// ```
///
/// The resulting string is `Hello World!`.
///
/// Note how the expression values to be formatted are inlined in the formatting braces.
/// Due to limitations of declarative macros the formatting braces are not part of the surrounding string literals.
///
/// ### Formatting specifiers
///
/// ```
/// # use format_xml::template;
/// let value = 42;
///
/// # let s =
/// template!("hex("{value}") = "{value;#x}).to_string()
/// # ;assert_eq!(s, "hex(42) = 0x2a");
/// ```
///
/// The resulting string is `hex(42) = 0x2a`.
///
/// Due to limitations of declarative macros, a semicolon is used to separate the value from the formatting specifiers.
/// The rules for the specifiers are exactly the same as Rust's standard formatting syntax.
///
/// ### Control flow
///
/// ```
/// # use format_xml::template;
/// let switch = true;
/// let opt = Some("World");
/// let result: Result<f32, i32> = Err(13);
///
/// # let s =
/// template! {
/// 	if let Some(name) = (opt) {
/// 		"# Hello " {name}"\n\n"
/// 	}
/// 	else if (switch) {
/// 		"# Hello User\n\n"
/// 	}
/// 	if (switch) {
/// 		"Message: "
/// 		match (result) {
/// 			Ok(f) => { "_"{f}"_\n\n" }
/// 			Err(i) => { "**"{i}"**\n\n" }
/// 		}
/// 		for i in (1..=5) {
/// 			let times_five = i * 5;
/// 			"* "{i}" x 5 = "{times_five}"\n"
/// 		}
/// 	}
/// 	else {
/// 		"No contents"
/// 	}
/// }.to_string()
/// # ;assert_eq!(s, "# Hello World\n\nMessage: **13**\n\n* 1 x 5 = 5\n* 2 x 5 = 10\n* 3 x 5 = 15\n* 4 x 5 = 20\n* 5 x 5 = 25\n");
/// ```
///
/// The resulting string is `# Hello World\n\nMessage: **13**\n\n* 1 x 5 = 5\n* 2 x 5 = 10\n* 3 x 5 = 15\n* 4 x 5 = 20\n* 5 x 5 = 25\n`.
///
/// ### Custom formatting
///
/// ```
/// # use format_xml::template;
/// # let s =
/// template! { |f| {
/// 	f.write_str("custom formatting")
/// }}.to_string()
/// # ;assert_eq!(s, "custom formatting");
/// ```
///
/// If all else fails the closure syntax enables to write custom formatting code.
/// The signature is the same as the `fmt` method of Rust's standard formatting traits.
///
/// `f` is [`&mut Formatter`](std::fmt::Formatter) and the closure returns a [`Result`](std::fmt::Result).
#[macro_export]
macro_rules! template {
	($($tt:tt)*) => { format_args!("{}", $crate::fmt(|_f| { $crate::_template_!({_f} $($tt)*); Ok(()) })) };
}

#[macro_export]
#[doc(hidden)]
macro_rules! _template_ {
	// text
	({$f:ident $($stmt:stmt)*} $text1:literal $text2:literal $($tail:tt)*) => {
		$crate::_template_text_!({$f $($stmt)*} concat!($text1, $text2), $($tail)*)
	};
	({$f:ident $($stmt:stmt)*} $text:literal $($tail:tt)*) => {
		$crate::_template_!({$f $($stmt)* $f.write_str(concat!($text))?} $($tail)*)
	};
	// format
	({$f:ident $($stmt:stmt)*} {$e:expr} $($tail:tt)*) => {
		$crate::_template_!({$f $($stmt)* $f.write_fmt(format_args!("{}", $e))?} $($tail)*)
	};
	({$f:ident $($stmt:stmt)*} {$e:expr;$($s:tt)*} $($tail:tt)*) => {
		$crate::_template_!({$f $($stmt)* $f.write_fmt(format_args!(concat!("{:", $(stringify!($s),)* "}"), $e))?} $($tail)*)
	};
	({$f:ident $($stmt:stmt)*} |$ff:ident| { $($body:stmt)* } $($tail:tt)*) => {
		$crate::_template_!({$f $($stmt)* { let $ff = &mut *$f; $($body)* }?} $($tail)*)
	};
	// control
	({$f:ident $($stmt:stmt)*} let $p:pat = $e:expr; $($tail:tt)*) => {
		$crate::_template_!({$f $($stmt)* let $p = $e} $($tail)*)
	};
	({$f:ident $($stmt:stmt)*} if ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_template_cond_!({$f $($stmt)*} [if $e { $crate::_template_!({$f} $($body)*); }] $($tail)*)
	};
	({$f:ident $($stmt:stmt)*} if let $p:pat = ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_template_cond_!({$f $($stmt)*} [if let $p = $e { $crate::_template_!({$f} $($body)*); }] $($tail)*)
	};
	({$f:ident $($stmt:stmt)*} match ($e:expr) { $($p:pat => { $($body:tt)* }$(,)?)* } $($tail:tt)*) => {
		$crate::_template_!({$f $($stmt)* match $e { $($p => { $crate::_template_!({$f} $($body)*); },)* }} $($tail)*)
	};
	({$f:ident $($stmt:stmt)*} for $p:pat in ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_template_!({$f $($stmt)* for $p in $e { $crate::_template_!({$f} $($body)*); }} $($tail)*)
	};
	// term
	({$f:ident $($stmt:stmt)*}) => {
		$($stmt)*
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _template_text_ {
	({$f:ident $($stmt:stmt)*} concat!($($texts:literal),*), $text:literal $($tail:tt)*) => {
		$crate::_template_text_!({$f $($stmt)*} concat!($($texts,)* $text), $($tail)*)
	};
	({$f:ident $($stmt:stmt)*} concat!($($texts:literal),*), $($tail:tt)*) => {
		$crate::_template_!({$f $($stmt)* $f.write_str(concat!($($texts),*))?} $($tail)*)
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! _template_cond_ {
	({$f:ident $($stmt:stmt)*} [$($c:tt)*] else if ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_template_cond_!({$f $($stmt)*} [$($c)* else if $e { $crate::_template_!({$f} $($body)*); }] $($tail)*)
	};
	({$f:ident $($stmt:stmt)*} [$($c:tt)*] else if let $p:pat = ($e:expr) { $($body:tt)* } $($tail:tt)*) => {
		$crate::_template_cond_!({$f $($stmt)*} [$($c)* else if let $p = ($e) { $crate::_template_!({$f} $($body)*); }] $($tail)*)
	};
	({$f:ident $($stmt:stmt)*} [$($c:tt)*] else { $($body:tt)* } $($tail:tt)*) => {
		$crate::_template_!({$f $($stmt)* $($c)* else { $crate::_template_!({$f} $($body)*); }} $($tail)*)
	};
	({$f:ident $($stmt:stmt)*} [$($c:tt)*] $($tail:tt)*) => {
		$crate::_template_!({$f $($stmt)* $($c)*} $($tail)*)
	};
}
