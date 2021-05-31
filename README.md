
These are examples I've shown in class, and maybe others.

The Rust examples are in folders. Just `cd` into the folder and `cargo run` to test, and look in the `src` folder for the code.

**Be sure to thoroughly read the comments in the source code.** I explain a lot of stuff there.

### Rust

- `rust_vecs/`
	- Creating, passing, returning, iterating, and slicing `Vec`s.
- `rust_structs/`
	- How Rust `struct`s and `impl` work.
- `rust_enums/`
	- An example of `enum`s and `struct`s, and it also shows `match`.
- `rust_trees/`
	- `Option<Box<...>>` makes it possible to make tree data structures.

### Lexing

- `StringWeirdness.java`
	- Two similar-looking strings, but they have different properties.
	- The strings are *not* the same:
		- `String s` has a precomposed `é` (`U+00E9`).
		- `String t` has a regular `e` (`U+0065`) followed by a combining mark (`U+0301`).
- `lexing_toy/`
	- A very simple lexer for a language composed of just parentheses, identifiers, and base-10 int literals.
	- `cargo run` gives you an interactive prompt to type code, and it shows the tokens for that code.

### ASTs/Parsing

- `ast_math/`
	- Demonstrates a simple mathematical AST that can be displayed and even evaluated.
- `parsing_lisp/`
	- A **recursive-descent** parser that parses a very simplified Lisp dialect.