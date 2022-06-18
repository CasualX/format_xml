/*!
Replace the standard formatting macros using [xfmt syntax](crate::xfmt!).
*/

/// Replaces `print!` using [xfmt syntax](crate::xfmt!).
#[cfg(feature = "std")]
#[macro_export]
macro_rules! print {
	($($tt:tt)*) => {
		::std::print!("{}", $crate::fmt(|_f| {
			$crate::__xfmt!{_f concat() $($tt)*}
			Ok(())
		}))
	};
}

/// Replaces `println!` using [xfmt syntax](crate::xfmt!).
#[cfg(feature = "std")]
#[macro_export]
macro_rules! println {
	($($tt:tt)*) => {
		::std::print!("{}", $crate::fmt(|_f| {
			$crate::__xfmt!{_f concat() $($tt)* "\n"}
			Ok(())
		}))
	};
}

/// Replaces `eprint!` using [xfmt syntax](crate::xfmt!).
#[cfg(feature = "std")]
#[macro_export]
macro_rules! eprint {
	($($tt:tt)*) => {
		::std::eprint!("{}", $crate::fmt(|_f| {
			$crate::__xfmt!{_f concat() $($tt)*}
			Ok(())
		}))
	};
}

/// Replaces `eprintln!` using [xfmt syntax](crate::xfmt!).
#[cfg(feature = "std")]
#[macro_export]
macro_rules! eprintln {
	($($tt:tt)*) => {
		::std::eprint!("{}", $crate::fmt(|_f| {
			$crate::__xfmt!{_f concat() $($tt)* "\n"}
			Ok(())
		}))
	};
}

/// Replaces `write!` using [xfmt syntax](crate::xfmt!).
#[macro_export]
macro_rules! write {
	($dst:expr, $($tt:tt)*) => {
		::core::write!($dst, "{}", $crate::fmt(|_f| {
			$crate::__xfmt!{_f concat() $($tt)*}
			Ok(())
		}))
	};
}

/// Replaces `writeln!` using [xfmt syntax](crate::xfmt!).
#[macro_export]
macro_rules! writeln {
	($dst:expr, $($tt:tt)*) => {
		::core::write!($dst, "{}", $crate::fmt(|_f| {
			$crate::__xfmt!{_f concat() $($tt)* "\n"}
			Ok(())
		}))
	};
}

/// Replaces `format!` using [xfmt syntax](crate::xfmt!).
#[cfg(feature = "std")]
#[macro_export]
macro_rules! format {
	($($tt:tt)*) => {
		::std::format!("{}", $crate::fmt(|_f| {
			$crate::__xfmt!{_f concat() $($tt)*}
			Ok(())
		}))
	};
}

/// Replaces `format_args!` using [xfmt syntax](crate::xfmt!).
#[macro_export]
macro_rules! format_args {
	($($tt:tt)*) => {
		::core::format_args!("{}", $crate::fmt(|_f| {
			$crate::__xfmt!{_f concat() $($tt)*}
			Ok(())
		}))
	};
}

/// Replaces `panic!` using [xfmt syntax](crate::xfmt!).
#[macro_export]
macro_rules! panic {
	($($tt:tt)*) => {
		::core::panic!("{}", $crate::fmt(|_f| {
			$crate::__xfmt!{_f concat() $($tt)*}
			Ok(())
		}))
	};
}

#[test]
fn test_prelude() {
	use std::fmt::Write;
	crate::print!(<a>"print"</a>);
	crate::println!(<a>"println"</a>);
	crate::eprint!(<a>"eprint"</a>);
	crate::eprintln!(<a>"eprintln"</a>);
	let mut s = crate::format!(<a>"format"</a>);
	let _ = crate::write!(s, <a>"write"</a>);
	let _ = crate::writeln!(s, <a>"writeln"</a>);
	assert_eq!(s, "<a>format</a><a>write</a><a>writeln</a>\n");
	// panic!(<a>"panic"</a>);
}
