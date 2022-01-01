// More on strings: https://doc.rust-lang.org/book/ch08-02-strings.html

fn main() {
    // Notice that we don't own this data. We cannot change it.
    let important_order = "Execute order 66!";

    // This operation doesn't modify anything. Rather, it simply returns a
    // new String, where the pattern "66" has been replaced by "65".
    let new_order = important_order.replace("66", "65");

    // This will output "Execute order 66! | Execute order 65!"
    println!("{} | {}", important_order, new_order);


    // We can create a new String from a string literal.
    // let new_string = String::from("hey.");

    let weird_string = create_weird_string(&new_order);
    
    // This will output "Execute xxdex 65!"
    println!("{}", weird_string);

    /* NOTE: Rust doesn't support String indexing, as Strings are not stored as chars,
       but as bytes. 
    */
    // This line wouldn't work.
    //let s = weird_string[0];

    // String concatenation.
    let result = concatenate("Kenobi!");

    // Will output "General Kenobi!"
    println!("{}", result);


    // Using the + operator to concatenate several strings can make the code hard to read.
    // To tidy up our code, we can use the format! macro:
    let cooler_result = format!("{} {}", "General", "Kenobi!");

    // Will also output "General Kenobi!".
    println!("{}", cooler_result);

}


// Appends every char of s to the end of the string, then returns it.
// Yes, I am aware. This is not the most efficient way to do this.
fn create_weird_string(s: &str) -> String {
    // Creates an empty string with the same size as s, to prevent resizing operations.
    let mut string = String::with_capacity(s.len());

    // Iterate through each character in the string.
    for mut i in s.chars() {
        if i == 'o' || i == 'r' {
            i = 'x';
        }
        string.push(i);
    }

    return string;
}

// This function attempts to concatenate two strings together.
fn concatenate(s: &str) -> String {
    let s2 = String::from("General ");

    // Takes ownership of s2 and appends a copy of s, returning ownership of the result.
    let result = s2 + &s;

    // This wouldn't work, as we're trying to borrow a moved value.
    // println!("{}", s2);

    return result;
}