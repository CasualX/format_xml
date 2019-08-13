/*!

!*/

#![feature(slice_patterns)]

extern crate proc_macro;
use proc_macro::*;

trait ToVec: Iterator {
	fn to_vec(self) -> Vec<Self::Item>;
}
impl<I: Iterator> ToVec for I {
	fn to_vec(self) -> Vec<Self::Item> {
		self.collect()
	}
}

#[proc_macro]
pub fn format_xml(args: TokenStream) -> TokenStream {
	let args = args.into_iter().to_vec();

	let mut fmt = Format::default();
	process(&args, &mut fmt);

	let string = TokenTree::Literal(Literal::string(&fmt.string));
	let mut params = vec![string];
	for arg in fmt.args {
		params.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
		params.push(TokenTree::Group(Group::new(Delimiter::None, arg)));
	}
	let result: TokenStream = vec![
		TokenTree::Ident(Ident::new("format_args", Span::call_site())),
		TokenTree::Punct(Punct::new('!', Spacing::Alone)),
		TokenTree::Group(Group::new(Delimiter::Parenthesis, params.into_iter().collect())),
	].into_iter().collect();
	// panic!(result.to_string());
	result
}

#[derive(Clone, Default)]
struct Format {
	string: String,
	args: Vec<TokenStream>,
}

fn process(mut tokens: &[TokenTree], format: &mut Format) {
	while tokens.len() > 0 {
		match tokens.get(0) {
			Some(TokenTree::Punct(punct)) => {
				if punct.as_char() == '<' {
					process_tag(&mut tokens, format);
				}
				else {
					panic!("Unexpected punct `{}`, write a literal instead", punct.as_char());
				}
			},
			Some(TokenTree::Literal(lit)) => {
				process_lit(&lit, format);
				tokens = &tokens[1..];
			},
			Some(TokenTree::Group(group)) => {
				match group.delimiter() {
					Delimiter::Brace => {
						process_arg(group.stream(), format);
						tokens = &tokens[1..];
					},
					_ => panic!("Format args must be grouped by braces `{}`"),
				}
			},
			Some(TokenTree::Ident(ident)) => {
				panic!("Unexpected ident `{}`, write a literal instead", ident.to_string());
			},
			None => break,
		}
	}
}

fn process_tag(tokens: &mut &[TokenTree], format: &mut Format) {
	*tokens = &(*tokens)[1..];

	let mut closing = false;
	format.string.push_str("<");
	if let Some(TokenTree::Punct(punct)) = tokens.get(0) {
		if punct.as_char() == '/' {
			*tokens = &(*tokens)[1..];
			closing = true;
			format.string.push_str("/");
		}
	}
	
	if let Some(tag_name) = parse_ident(tokens) {
		for item in tag_name {
			format.string.push_str(&item.to_string());
		}
	}

	if !closing {
		while process_attr(tokens, format) {}
	}

	match *tokens {
		[TokenTree::Punct(punct), ..] if punct.as_char() == '>' => {
			format.string.push_str(">");
			*tokens = &(*tokens)[1..];
		},
		[TokenTree::Punct(p1), TokenTree::Punct(p2), ..] if p1.as_char() == '/' || p2.as_char() == '>' => {
			format.string.push_str(" />");
			*tokens = &(*tokens)[2..];
		},
		_ => panic!("expecting '>' closing bracket"),
	}
}
fn process_attr(tokens: &mut &[TokenTree], format: &mut Format) -> bool {
	process_ident(tokens, format);
	match parse_ident(tokens) {
		Some(attr_name) => {
			format.string.push_str(" ");
			for item in attr_name {
				format.string.push_str(&item.to_string());
			}

			match tokens.get(0) {
				Some(TokenTree::Punct(punct)) if punct.as_char() == '=' => {
					format.string.push_str("=");
					*tokens = &(*tokens)[1..];

					match tokens.get(0) {
						Some(TokenTree::Literal(lit)) => {
							format.string.push_str("\"");
							process_lit(&lit, format);
							format.string.push_str("\"");
						},
						Some(TokenTree::Group(group)) => {
							format.string.push_str("\"");
							process_arg(group.stream(), format);
							format.string.push_str("\"");
						},
						_ => panic!("expected literal or group"),
					}

					*tokens = &(*tokens)[1..];
					return true;
				},
				Some(TokenTree::Punct(punct)) if punct.as_char() == '>' => {
					return false;
				},
				_ => panic!("expected '=' or '>'"),
			}
		},
		_ => return false,
	}
}
fn process_lit(lit: &Literal, format: &mut Format) {
	let string = lit.to_string();
	let mut string = &string[..];

	if string.len() >= 2 && string.starts_with("\"") && string.ends_with("\"") {
		string = &string[1..string.len() - 1];
	}

	let escaped = string
		.replace("&", "&amp;")
		.replace("<", "&lt;")
		.replace(">", "&gt;")
		.replace("\"", "&quot;")
		.replace("\'", "&apos;");

	format.string.push_str(&escaped);
}
fn process_arg(stream: TokenStream, format: &mut Format) {
	// Split the format specifiers from the stream
	let tokens = stream.into_iter().to_vec();
	let rpos = tokens.iter().rposition(|token| match token {
		TokenTree::Punct(punct) if punct.as_char() == ':' => true,
		_ => false,
	});
	if let Some(index) = rpos {
		format.string.push_str("{:");
		for token in &tokens[index + 1..] {
			format.string.push_str(&token.to_string());
		}
		format.string.push_str("}");
		format.args.push(tokens[..index].iter().cloned().collect());
	}
	else {
		format.string.push_str("{}");
		format.args.push(tokens.into_iter().collect());
	}
}
fn process_name(tokens: &mut &[TokenTree], format: &mut Format) -> bool {
	match tokens.get(0) {
		Some(TokenTree::Ident(ident)) => {
			format.string.push_str(&ident.to_string());
			*tokens = &(*tokens)[1..];
		}
		_ => return true,
	}
	loop {
		match tokens.get(0) {
			Some(TokenTree::Punct(punct)) => {
				if punct.as_char() == '-'
				format.string.push_str(&punct.to_string());
				*tokens = &(*tokens)[1..];
			},
			_ => return false,
		}
		match tokens.get(0) {
			Some(TokenTree::Ident(ident)) => {
				format.string.push_str(&ident.to_string());
				*tokens = &(*tokens)[1..];
			}
			_ => return false,
		}
	}
}
fn process_