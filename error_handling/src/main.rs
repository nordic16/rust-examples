// More on error handling: https://doc.rust-lang.org/book/ch09-00-error-handling.html

use std::fs::File;
use std::io::Read;

fn main() {
    // If we change this variable to some other value, the code will panic.
    let filename = "super_secret.txt";
    let s = read_from_file(&filename).expect("Couldn't get the super secret key :(\n");

    println!("{}", s);
}


fn read_from_file(filename: &str) -> Result<String, std::io::Error> {
    let mut result = String::new();

    // The '?' operator is essentially a shorthand for a match expression that handles the error.
    // It returns the error back to the caller: This is called error propagation, giving the caller more control.
    // This operator can only be used inside a function that returns a Result.
    File::open(&filename)?.read_to_string(&mut result)?;

    // Is the same as this:
    /* 
    let mut file = match File::open(&filename) {
        Ok(s) => s,
        Err(e) => return Err(e), 
    };
    */

    // This code can be made even simpler :P
    // Just use the one liner std::fs::read_to_string(&filename)

    return Ok(result);
}