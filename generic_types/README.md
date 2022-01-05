# Generic Types - Less Code, More Efficiency.

## What are the generic types?
Sometimes we want to execute code for a certain type, such as an `i32`. To accomplish that, we define a method/function with the required parameters.
But sometimes we also want to execute code that works with different types, such as an `i32` or `char`. That's what generic types are for.

* In the following example: 
    * `T` is out type parameter. It can only be used by types that implement both the `PartialOrd` and `Copy` traits.
    * `list` is of type `&Vec<T>`, meaning that it can be a vector of anything that matches the above condition.


### Example:
```rust
fn largest<T: PartialOrd + Copy>(list: &Vec<T>) -> &T {
    let mut max = &list[0];

    for i in list {
        if i > max {
            max = i;
        }
    }
    max
}
```

* With generic types, we can prevent code duplication. Without them, we'd have to write this function again only to operate on, say,  a `Vec<f32>`!

## Generic types on structs/enums.
We can also use the power of generics on structs and/or enums! In fact, we've been using them all this time, on types such as `Vec<T>`, `Option<T>` and even `Result<T, E>`!

### Example:
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    //...
}
```

* Note that `x` and `y` must be of the same type.

```rust
enum Option<T> {
    Some(T),
    None
}
```

## Generic types and performance
If this is not the first time you've been exposed to generic types, you might already know that normally there's a runtime cost when using generic types. This is not the case in Rust:
* At compile time, Rust performs monomorphization, which is the process of generating code that uses concrete types instead of abstract types.
* According to the Rustlang book:
    > "Rust accomplishes this by performing monomorphization of the code that is using generics at compile time. Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled."