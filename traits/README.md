
# Traits - Sharing functionality.

## What are traits?
If you've ever used any other programming language, such as Java, you can think of traits like you think of interfaces -- they're groups of methods that can be implemented by structs in order to share functionality. 

### An example:
```rust
// vehicle/vehicles.rs
pub trait Vehicle {
	// Note the semicolon at the end.
	pub fn set_speed(&self, speed: i32);
    pub fn stop(&self);
    pub fn brake(&self);
}

pub struct Car {
    brand: String,
}

pub struct Bike {
    brand: String,
}

// to implement all methods of Vehicle, we use the following syntax:
impl Vehicle for Car {
	fn brand(&self) -> String {
		brand
	}
	// Now Car has all of Vehicle's functions, but we must define their bodies...
}

// impl block for bike......
```

* It is possible to define a function inside a trait. That way, we don't have to define that function every time its trait is implemented.

* Traits can be used in function, as parameters:
	```rust
	fn some_function(type: impl Trait) {
	//...
	}
	```

	* Note, though, that this is just syntax sugar for the following:
	```rust
	fn some_function<T: Trait>(type: &T) {
		//...
	}
	```

	* This is called a trait bound.
	* Sometimes trait bounds can be hard to read. This is where `where` clauses come in handy.
	````rust
	fn some_function<T, U>(type: &t, u: &U) 
		where T: Trait + Display,
			  U: Trait + Display
	{
		//...
	}

* We can also return any type that implements a trait through the `impl Trait` syntax.
* Traits can also be generic. `trait<T> Trait.`
* Last but not the least, we can also conditionally implement methods only when a given type implements a certain trait:
	```rust
	use std::fmt::Display;

	struct Pair<T> {
		x: T,
		y: T,
	}

	impl<T> Pair<T> {
		fn new(x: T, y: T) -> Self {
			Self { x, y }
		}
	}

	// methods and functions inside this impl block will only be available for types that implement Display and PartialOrd.
	impl<T: Display + PartialOrd> Pair<T> {
		fn cmp_display(&self) {
			if self.x >= self.y {
				println!("The largest member is x = {}", self.x);
			} else {
				println!("The largest member is y = {}", self.y);
			}
		}
	}
	```

## Restrictions of traits:
* Either the trait to be implemented or the type that is to implement it must be local to a crate. This restriction is due to a concept called coherence, that prevents code breaking changes.
* The `impl Trait` syntax can only be used with functions that return a single type. 