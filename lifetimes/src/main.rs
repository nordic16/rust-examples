fn main() {
    let s = "general kenobi!";
    let s2 = "hello there";

    println!("Longest: {}", longest(s, s2));
}


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x
    }
}


// This function still works, thanks to lifetime ellision rules!
fn first_word(x: &str) -> &str {
    let bytes = x.as_bytes();

    for (i, &c) in bytes.iter().enumerate() {
        if c == b' ' {
            return &x[0..i];
        }
    }
    &x[..]
}