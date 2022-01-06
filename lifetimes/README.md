# Lifetimes - Validating references has never been easier!
All variables in Rust have lifetimes, which is nothing less than their scope.

## What exactly are lifetimes for?
As we know by this point, Rust prevents code with dangling references to compile. This happens, for example, when we try to access memory that has already been deallocated.
```rust
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          | <- ERROR: x doesn't live enough.
    }                         // ---------+
```

* As we can see in the example above, the lifetime of `x` (`'b`) is shorter than `r`'s, therefore this could would create a dangling reference. Rust knows this through its borrow checker, that checks all scopes involved to determine if all borrows are valid.

## Using lifetimes:
### In functions:
Let's take a look at the following example:
```rust
fn longest(x: &str, y: &str) -> &str { <- ERROR: Expected named lifetime parameter.
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```


There are two reasons why we need to explicitly define a lifetime: 
* Rust doesn't know whether the return type borrows from `x` or `y` (and neither do we).
* We don't know the concrete lifetime of the variables that will be passed onto this function, so we can look at their scopes.

Now, in order to fix this issue, all we need is to add a named lifetime parameter:
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
* Note: We call lifetimes of arguments passed in input lifetimes, while lifetimes of return values are called output lifetimes.

* Lifetime parameters don't really change for how long variables live for. In the example above, we use `'a`, the smaller of the lifetimes between two concrete references such as `x` and `y`, to tell the borrow checker that the returned value's lifetime is the smaller of the lifetimes between `x` and `y`. These constraints are what we want the borrow checker to enforce.

```rust
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result); // ERROR: string2 doesn't live enough.
}
```

* Since the lifetime of `result` is equal to the smaller of the two concrete reference's lifetimes (`string2`), this code doesn't work. 
* Note that if `longest()` returned a reference to `string1`, it'd still be valid, but the borrow checker disallows this code because of the *possibility* of returning a dangling reference.

If we were to only return the first parameter inside `longest()`, we wouldn't need the lifetime parameter on `y`, because the lifetime of `y` wouldn0t have any relationship with the lifetime of `x` or the return value.


### In structs:
```rust
struct Book<'a> {
    title: &'a str,
}
```

* This notation simply means that an instance of `Book` cannot outlive the reference it holds (`title`).

## Lifetime ellision rules:
Sometimes, the compiler can infer the lifetime annotations. It does this by checking the 3 ellision rules.
1. Each parameter that's a reference gets its own lifetime parameter.
2. If there's exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
3. In case of multiple input lifetime parameters, but one of them is `&self` or `&mut self` the lifetime of self is assigned to all output lifetime parameters.

## The static lifetime.
A `'static` lifetime means that the reference could live as long as the duration of the process. Recall that string literals are stored in the binary. Well, they also have a static lifetime!