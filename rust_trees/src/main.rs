#[derive(Debug)]
struct Node<T> {
	// Option<> makes it nullable (can be None); Box<> makes it a reference.
	// It looks verbose, but you rarely have to actually type this type.
	left:  Option<Box<Node<T>>>,
	right: Option<Box<Node<T>>>,
	value: T,
}

impl<T> Node<T> {
	// A convenience constructor to make a new boxed node with no children.
	fn new(value: T) -> Box<Self> {
		Box::new(Self {
			value,
			left: None,
			right: None,
		})
	}
}

fn main() {
	let mut a = Node::new(5);
	a.left  = Some(Node::new(2));
	a.right = Some(Node::new(7));
	println!("{:#?}", a);
}