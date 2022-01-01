# Error handling in Rust.

## Types of errors.
One thing Rust does different compared to many other programming languages is that, contrary to other languages, it doesn't have exceptions: Rust divides errors between two main categories:

* Recoverable errors: Errors that don't make your program crash.
* Unrecoverable errors: Errors that, as the name suggests, force the program to crash. They're usually thrown through the `panic!` macro. 


## How to work with errors in Rust
Just like we used an enum, `Option<T>`, to represent the possibility of a value being absent, we're going to use an enum: `Result<T, E>`, where `T` is the value returned in case of success and `E` is the error in case of failure.

### Example of unrecoverable error:
```rust
fn crash_if_bad_number(x: i32) {
	if (x == 32) {
		panic!("Welp, my program just crashed.");
	}
}
```

### Example of recoverable error:
```rust
fn read_from_file(f: &str) -> Result<String, std::io::error> {
	let mut file = std::fs::File::open(&f)?; // More on the '?' operator in main.rs.
	let mut result = String::new();
	
	// Wrap the result around Ok().
	return Ok(file.read_to_string(&mut result))?;
}
```

Then, to retrieve the result:
```rust
// In case of success, retrieves the result of this operation (String), otherwise panics.
let cool_string = read_from_file("hello_there").unwrap();

// Works just like the line above, but in case of failure panics with the message inside expect().
let cool_string2 = read_from_file("hello_there").expect("No General Kenobi in here.");
```

* `expect()` and `unwrap()` are particularly useful when testing code and error handling is not yet at hand or when we're certain a call to a function will succeed. 

## panic! vs Result: Battle of the Heroes
* Generally, one should use `Result<>` by default, since it provides more flexibility to the caller. Unless it is impossible to recover from the state a program may be in, `Result<>` should be preffered over `panic!`.
