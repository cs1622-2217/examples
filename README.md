
These are examples I've shown in class, and maybe others.

The Rust examples are in folders. Just `cd` into the folder and `cargo run` to test, and look in the `src` folder for the code.

**Be sure to thoroughly read the comments in the source code.** I explain a lot of stuff there.

- `StringWeirdness.java`
	- Two similar-looking strings, but they have different properties.
	- The strings are *not* the same:
		- `String s` has a precomposed `é` (`U+00E9`).
		- `String t` has a regular `e` (`U+0065`) followed by a combining mark (`U+0301`).