use std::fmt;

/// Formats tuples as a comma separated items.
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct CommaFmt<T>(pub T);
impl<T: fmt::Display, U: fmt::Display> fmt::Display for CommaFmt<(T, U)> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_fmt(format_args!("{},{}", (self.0).0, (self.0).1))
	}
}
impl<T: fmt::Display, U: fmt::Display, V: fmt::Display> fmt::Display for CommaFmt<(T, U, V)> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_fmt(format_args!("{},{},{}", (self.0).0, (self.0).1, (self.0).2))
	}
}
impl<T: fmt::Display> fmt::Display for CommaFmt<[T; 2]> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_fmt(format_args!("{},{}", (self.0)[0], (self.0)[1]))
	}
}
impl<T: fmt::Display> fmt::Display for CommaFmt<[T; 3]> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_fmt(format_args!("{},{},{}", (self.0)[0], (self.0)[1], (self.0)[2]))
	}
}

/// Formats an iterable with spaces between the items.
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct SpacedSet<I>(pub I);
impl<T: fmt::Display, I: Clone + IntoIterator<Item = T>> fmt::Display for SpacedSet<I> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for item in self.0.clone() {
			f.write_fmt(format_args!("{} ", item))?;
		}
		Ok(())
	}
}

/// Formats an iterable with spaces of comma formatted items.
///
/// ```
/// let data = [(1.0, 2.0), (13.0, 42.0), (-5.0, 100.0)];
/// let result = format_xml::xml! {
/// 	<polygon points={format_xml::spaced_comma_set(data.iter().cloned())} />
/// }.to_string();
/// assert_eq!(result, r#"<polygon points="1,2 13,42 -5,100 " />"#);
/// ```
pub fn spaced_comma_set<I: IntoIterator>(iterable: I) -> SpacedSet<impl Clone + Iterator<Item = CommaFmt<I::Item>>> where I::IntoIter: Clone {
	SpacedSet(iterable.into_iter().map(CommaFmt))
}
