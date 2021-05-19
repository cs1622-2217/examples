// #[derive] lets the compiler automatically write some methods for our custom types.
// - Debug will let us print out Points using {:?}.
// - PartialEq and Eq will let us compare Points with the == and != operators.
#[derive(Debug, PartialEq, Eq)]
struct Point {
	x: i32,
	y: i32,
}

// this says, "associate these functions with the Point type."
impl Point {
	// The "constructor". Not built into the language, but a common pattern.
	// no &self argument, which means this is like a "static method" and will
	// be called as Point::new(x, y).
	fn new(x: i32, y: i32) -> Point {
		// "return" is optional in Rust. the last expression will be
		// returned, as long as you don't put a ; at the end.
		Point { x, y }
	}

	// &self - this is a regular method. &self methods cannot modify the object
	// that they are called on.
	fn flip_x(&self) -> Point {
		// We return a *new* Point here because we can't change self.
		Point::new(-self.x, self.y)
	}

	// "&mut self" makes a method that can also change the struct's fields.
	// This kind of method can only be called on mut variables.
	fn flip_x_in_place(&mut self) {
		self.x = -self.x;
	}
}

fn main() {
	// We can create a Point without the constructor like this:
	let p = Point { x: 3, y: 4 };
	println!("      struct: {:?}", p);

	// fields are accessed with . like Java.
	println!("  its fields: x = {}, y = {}", p.x, p.y);

	// And using the constructor like this:
	let p = Point::new(7, 8);

	// We can call "methods" on structs just like in Java.
	let fp = p.flip_x();
	println!("    original: {:?}", p);
	println!("     flipped: {:?}", fp);

	// Declaring a variable as "mut" lets us change its fields and call mut methods.
	let mut p = Point::new(9, 10);
	p.y = 20;
	p.flip_x_in_place();
	println!("     mutable: {:?}", p);

	// trying to change fields or call mut methods on an immutable variable
	// will give a compiler error. uncomment these to see.
	// fp.y = 30;
	// fp.flip_x_in_place();
}