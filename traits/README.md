
# Traits - Sharing functionality.

## What are traits?
If you've ever used any other programming language, such as Java, you can think of traits like you think of interfaces -- they're groups of methods that can be implemented by structs in order to share functionality. 

## An example:
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

* It is possible to define a function inside a trait. That way, we don't have to define that function every time it is implemented.

