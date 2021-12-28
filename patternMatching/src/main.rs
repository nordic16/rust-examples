fn main() {
    // In Rust, there is only one way to specify absent (null) values: Through the Option<T> enum.
    // Although explicitly naming the variable type isn't needed in this case, I recommend it, as it makes the code easier to read.
    let cool_number: Option<i32> = Some(16);

    // Will turn everyone you know against you! Use at your own risk.
    let order: Option<i32> = Some(66);

    // This variable doesn't hold anything.
    let absent_number: Option<i32> = None;

    // This would cause the program to crash, as it's trying to retrieve a value that doesn't exist.
    // println!("{}", absent_number.unwrap());

    // Try to guess what will be the output for each of these function calls!
    print_value(&cool_number);
    print_value(&order);
    print_value(&absent_number);

    // As we've seen before, match statements in rust are exhaustive and can get quite verbose very quickly.
    // Since we only really need to check whether the value passed is not None, we can use if let to simplify our code.
    new_print_value(&cool_number);
    new_print_value(&order);
    new_print_value(&absent_number);
}

/*
 * Prints the value of x, and does nothing if x is None.
 */
fn print_value(x: &Option<i32>) {
    /* 
       Match statements are exhaustive, meaning that they must have an arm for every possible value. 
       In this case, there are only two possible values, though.
       In order to write an arm that matches every single possible value, we use the '_' operator: _ => [expression],
    */
    match x {
        Some(i) => println!("{}", i),
        None => (),
    }
}


// This code behaves exactly the same as the one above.
fn new_print_value(x: &Option<i32>) {
    /* 
       Friendly tip: Try to read this expression backwards.
       Also notice that we don't use '==', but '='.
       This syntax can be a bit confusing when getting started, but soon enough it will become intuitive.
    */
    if let Some(i) = x {
        println!("{}", i)
    }
}