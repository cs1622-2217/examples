
use std::fmt::{ Debug, Display, Formatter, Result as FmtResult };

// ------------------------------------------------------------------------------------------------
// Token type
// ------------------------------------------------------------------------------------------------

/*
These tokens would be produced by a lexer... IF WE HAD ONE
Well, another example has one. Maybe you can put the two examples together!
*/
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
	Eof,
	LParen,
	RParen,
	Id(String),
	IntLit(i64),
}

impl Display for Token {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		use Token::*;

		match self {
			Eof       => write!(f, ""),
			LParen    => write!(f, "("),
			RParen    => write!(f, ")"),
			Id(id)    => write!(f, "{}", id),
			IntLit(i) => write!(f, "{}", i),
		}
	}
}

// ------------------------------------------------------------------------------------------------
// AST type
// ------------------------------------------------------------------------------------------------

/*
Here is the full syntactic grammar for this language. Remember that the "alphabet" (set of symbols
that this grammar operates on) is *tokens* produced by the lexer. This is what the rules for Id,
Num, and Eof mean. '(' and ')' are also the LParen and RParen tokens, but to make the rules more
readable, we just write them explicitly.

	Program:  Exp Eof
	Exp:      Id | Num | ParenExp
	ParenExp: '(' Exp+ ')'

	Id:  <Id token from lexing phase>
	Num: <IntLit token from lexing phase>
	Eof: <'<eof>' token from lexing phase>
*/

#[derive(Clone)]
pub enum AstNode {
	Id(String),
	Num(i64),
	Exp(Vec<Box<AstNode>>),
}

impl AstNode {
	pub fn id(s: &str) -> Box<Self> {
		Box::new(AstNode::Id(s.into()))
	}

	pub fn num(i: i64) -> Box<Self> {
		Box::new(AstNode::Num(i))
	}

	pub fn exp(exps: Vec<Box<AstNode>>) -> Box<Self> {
		Box::new(AstNode::Exp(exps))
	}
}

// You can write your own implementations of Debug too, instead of #[derive]ing them.
// I'm doing this to make the output a little more compact than what #[derive] gives me.
impl Debug for AstNode {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		use AstNode::*;

		match self {
			Id(id) => write!(f, "Id({})", id),
			Num(i) => write!(f, "Num({})", i),
			Exp(exps) => {
				write!(f, "Exp")?;
				f.debug_list().entries(exps.iter()).finish()
			}
		}
	}
}

// ------------------------------------------------------------------------------------------------
// ParseError type
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum ParseError {
	ExpectedExpression,
	ExpectedLParen,
	ExpectedRParen,
	ExpectedEof,
}

impl Display for ParseError {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		use ParseError::*;

		match self {
			ExpectedExpression => write!(f, "expected an expression"),
			ExpectedLParen     => write!(f, "expected '(' to start an expression"),
			ExpectedRParen     => write!(f, "expected ')' to end an expression"),
			ExpectedEof        => write!(f, "expected end-of-file token at end of input"),
		}
	}
}

impl std::error::Error for ParseError {}

// ------------------------------------------------------------------------------------------------
// The recursive descent parser
// ------------------------------------------------------------------------------------------------

// `type` lets us make a shorthand alias for a longer type. Now wherever I write ParseResult,
// it's the same as writing Result<Box<AstNode>, ParseError>.
type ParseResult = Result<Box<AstNode>, ParseError>;

pub fn parse(tokens: &[Token]) -> ParseResult {
	let mut p = Parser::new(tokens);
	p.parse_program()
}

struct Parser<'t> {
	tokens: &'t [Token],
	pos:    usize,
}

impl<'t> Parser<'t> {
	fn new(tokens: &'t [Token]) -> Self {
		Parser { tokens, pos: 0 }
	}

	fn next(&mut self) {
		assert!(self.pos < self.tokens.len());
		self.pos += 1;
	}

	fn cur(&self) -> Token {
		if self.pos < self.tokens.len() {
			self.tokens[self.pos].clone()
		} else {
			Token::Eof
		}
	}

	// Program: Exp Eof
	fn parse_program(&mut self) -> ParseResult {
		let ret = self.parse_exp()?;
		self.expect_eof()?;
		Ok(ret)
	}

	// Exp: Id | Num | ParenExp
	fn parse_exp(&mut self) -> ParseResult {
		use Token::*;

		match self.cur() {
			Id(s)     => { self.next(); Ok(AstNode::id(&s)) }
			IntLit(i) => { self.next(); Ok(AstNode::num(i)) }
			LParen    => self.parse_paren_exp(),
			_         => Err(ParseError::ExpectedExpression),
		}
	}

	// ParenExp: '(' Exp+ ')'
	fn parse_paren_exp(&mut self) -> ParseResult {
		// Note the use of ? here. It means, "if expect_lparen() returned an error, then return
		// that error; otherwise, carry on as usual."
		self.expect_lparen()?;

		let mut exps = Vec::new();
		exps.push(self.parse_exp()?); // and here

		while self.cur() != Token::RParen {
			exps.push(self.parse_exp()?); // and here
		}

		self.expect_rparen()?; // and here!

		// and if we made it to the end of this method, everything is Ok()!
		Ok(AstNode::exp(exps))
	}

	// () is Rust's void.
	// This return type says "returns nothing on success, or ParseError on failure"
	fn expect_lparen(&mut self) -> Result<(), ParseError> {
		// Ok(()) is how you say "everything's Ok, but I don't have a value to return"
		match self.cur() {
			Token::LParen => { self.next(); Ok(()) }
			_             => Err(ParseError::ExpectedLParen),
		}
	}

	fn expect_rparen(&mut self) -> Result<(), ParseError> {
		match self.cur() {
			Token::RParen => { self.next(); Ok(()) }
			_             => Err(ParseError::ExpectedRParen),
		}
	}

	fn expect_eof(&mut self) -> Result<(), ParseError> {
		match self.cur() {
			Token::Eof => Ok(()),
			_          => Err(ParseError::ExpectedEof),
		}
	}
}