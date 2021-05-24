
use std::fmt::{ Display, Formatter, Result as FmtResult };

// ------------------------------------------------------------------------------------------------
// Token type
// ------------------------------------------------------------------------------------------------

/*
Token grammar:

LParen:  '('
RParen:  ')'
Id:      IdStart IdCont*
IdStart: <alphabetic> | '_'
IdCont:  IdStart | Digit
IntLit:  Digit+
Token:   LParen | RParen | Id | IntLit

Whitespace: ' ' | '\t' | '\n'
Program:    (Whitespace? Token)* Whitespace? Eof
*/

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenKind {
	Eof,
	LParen,
	RParen,
	Id(String),
	IntLit(i64),
}

#[derive(Debug, Clone)]
pub struct Token {
	pub loc:  usize, // the codepoint index in the source code.
	pub kind: TokenKind,
}

impl Token {
	// Self is a "magical type" that can be used in impl blocks, and refers to the type
	// that the impl is attached to. Here it means "Token".
	pub fn new(loc: usize, kind: TokenKind) -> Self {
		Token { loc, kind }
	}
}

// ------------------------------------------------------------------------------------------------
// LexError type
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum LexError {
	InvalidChar(char),
	IntOutOfRange,
}

impl Display for LexError {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		match self {
			LexError::InvalidChar(c) => write!(f, "invalid character '{}'", c.escape_debug()),
			LexError::IntOutOfRange  => write!(f, "integer out of range"),
		}
	}
}

impl std::error::Error for LexError {}

// ------------------------------------------------------------------------------------------------
// The lexer algorithm
// ------------------------------------------------------------------------------------------------

fn is_ident_start(c: char) -> bool { c.is_alphabetic() || c == '_' }
fn is_ident_cont (c: char) -> bool { is_ident_start(c) || c.is_ascii_digit() }

/*
Result<R, E> is how functions return errors in Rust. R is the return type if it succeeds; E is the
error type if it fails. So this function returns a Vec<Token> on success and a LexError on failure.
*/
pub fn lex(source: &str) -> Result<Vec<Token>, LexError> {
	// this turns source from a UTF-8 string into a vector of codepoints, so we can
	// index it by codepoint index in O(1) time.
	// (Rust lets you re-declare variables of the same name. It's really a new variable.)
	let source = source.chars().collect::<Vec<_>>();

	// our position in the source.
	let mut pos = 0;

	// the vec of tokens to be returned.
	let mut ret = vec![];

	// let's go! this loop implements the Program rule.
	while pos < source.len() {
		match source[pos] {
			// Whitespace
			' ' | '\t' | '\n' => {
				// ignore it and move on.
				pos += 1;
			}

			// LParen
			'(' => {
				ret.push(Token::new(pos, TokenKind::LParen));
				pos += 1;
			}

			// RParen
			')' => {
				ret.push(Token::new(pos, TokenKind::RParen));
				pos += 1;
			}

			// Id
			c if is_ident_start(c) => {
				let start = pos;
				let mut s = String::new();

				while pos < source.len() && is_ident_cont(source[pos]) {
					s.push(source[pos]);
					pos += 1;
				}

				ret.push(Token::new(start, TokenKind::Id(s)));
			}

			// IntLit
			c if c.is_ascii_digit() => {
				let start = pos;
				let mut num = String::new();

				while pos < source.len() && source[pos].is_ascii_digit() {
					num.push(source[pos]);
					pos += 1;
				}

				// this rule makes things like "123abc" invalid. this is actually
				// a lookahead because we're just checking the next character without
				// making it part of this token.
				if pos < source.len() && source[pos].is_alphabetic() {
					return Err(LexError::InvalidChar(source[pos]));
				}

				// some rules, like "can't exceed the capacity of a 64-bit integer," can't
				// be encoded in the grammar rules and have to be checked manually.
				match i64::from_str_radix(&num, 10) {
					Ok(value) => {
						ret.push(Token::new(start, TokenKind::IntLit(value)));
					}

					Err(..) => return Err(LexError::IntOutOfRange),
				}
			}

			c => return Err(LexError::InvalidChar(c))
		}
	}

	ret.push(Token::new(pos, TokenKind::Eof));

	// we indicate success by returning an Ok(..) value.
	Ok(ret)
}