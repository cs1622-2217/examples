
use ast_math::*;

fn main() {
	// an AST representing (3 + 5).
	let a = AstNode::add(AstNode::num(3.), AstNode::num(5.));
	show(&a);

	// let's make it bigger...
	let b = AstNode::neg(a);
	show(&b);

	// and bigger...
	let c = AstNode::div(AstNode::num(2.), b);
	show(&c);

	// BIGGER...
	let d = AstNode::mul(c, AstNode::num(-40.));
	show(&d);
}

// We take &Box and not just Box here because we just want to borrow
// the Box for a little while, and not take ownership of it from main.
fn show(ast: &Box<AstNode>) {
	println!("{}", ast);
	println!("=> {}", ast.eval());
	println!();
}