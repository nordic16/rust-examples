// For more information: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html

fn main() {
    let s: String = String::from("Hello there!");

    // takes ownership of s.
    takes_ownership(s);

    // This wouldn't compile, as s is no longer valid!
    // println!("The value of s value is: {}", s);

    // To fix this problem, we can refactor the takes_ownership function to borrow the value of name instead.
    let mut s1: String = String::from("General Kenobi");

    // Because this function takes a reference to s1 and not s1 itself, s1 will still be valid after this function is called.
    // In other words, this function borrows the value of s1.
    // Reference must be mut, as we're changing the value of s1 inside the function.
    borrows(&mut s1);

    // References go out of scope after the statement where they were last used.
    // The following statement would work perfectly fine.
    // let s = &s1;

    // Contrary to our first try, this will work as expected.
    println!("The value of s1 is: {}", s1);

    /* And one more thing: In order to make Rust a safe system-programming language, references have some limitations.
        - There can only be either one mutable reference (&mut x) or any number of unmutable references (&x).

        - Dangling references (references that don't point to anything) are prohibited in Rust. In C++, this used to be a common problem, but Rust ensures that no 
          code with dangling references compiles. (Whew!)
    */
}


// Whatever string is passed on to function, is dropped afterwards.
fn takes_ownership(name: String) {
    println!("{}", name);
}

// Borrows the value of whatever string is passed on to this function.
fn borrows(name: &mut String) {
    name.push_str(", you are a bold one!");
}

/*
    This code WON'T compile: It produces a dangling reference, as s is dealocated right after the return statement, producing a dangling reference.
fn returns_ownership() -> &String {
    let mut s:String = String::from("hi!");

    &s
}
*/
