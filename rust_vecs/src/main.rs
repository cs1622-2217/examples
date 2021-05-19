
fn main() {
	// constructing a new Vec is easy with the vec! macro.
	let mut vals = vec![1, 2, 3, 4, 5];

	// you can print Vecs with {:?}.
	println!("       vals: {:?}", vals);

	// the .len() method gets the length.
	println!("  vals' len: {}", vals.len());

	// .iter().enumerate() iterates over the indexes and the values of a vec.
	for (i, v) in vals.iter().enumerate() {
		println!("     vals[{}] = {}", i, v);
	}

	// now let's call that function.
	let doubled = double(&vals);
	println!("    doubled: {:?}", doubled);

	// since we declared it as 'mut', we can modify it too.
	vals[0] = 0;  // index it like an array
	vals.push(6); // append a value
	println!("vals is now: {:?}", vals);

	// last, we can get slices of vecs, a very useful feature.
	// this slice means "start at index 3, and go to the end."
	let slice = &vals[3 ..];
	println!("  vals[3..]: {:?}", slice);
}

// this *borrows* a Vec from the caller, and creates a new Vec whose contents
// are the original multiplied by 2.

// If the argument were "v: Vec<i32>", this function would "consume" the caller's
// Vec, and the above code would not work (because 'vals' would disappear!).
fn double(v: &Vec<i32>) -> Vec<i32> {
	// this weird style of writing code is pretty common when dealing with iterators.
	// we're just calling two methods on the iterator object, map and collect.
	v.iter()        // iterate over v
	.map(|x| x * 2) // ...multiply each value by 2
	.collect()      // ...and collect the values into a new Vec.
}