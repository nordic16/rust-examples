// More on generic types: https://doc.rust-lang.org/book/ch10-01-syntax.html

use rand::{thread_rng, Rng};

fn main() {

    let mut numbers: Vec<i32> = Vec::new();

    for _ in 0..15 {
        numbers.push(thread_rng().gen_range(0..=255));
    }

    println!("{:?}", numbers);
    println!("{}", largest(&numbers));

}


// This function would work for all types that implement both the PartialOrd and Copy traits.
fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> &T {
    let mut max = &list[0];

    for i in list {
        if i > max {
            max = i;
        }
    }
    max
}