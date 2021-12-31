# Strings
When you opened this folder, you were probably thinking: "Why are Strings even here? They're not so complicated!".
Unfortunately, they actually are that complicated. Rust does what many programming languages don't: Expose that very complexity to the programmer.

## How are they stored?
Strings in Rust are encoded in UTF-8 and stored as raw bytes, meaning that a character may span from **1** to **4** bytes. This is particularly important, as trying to read, say, the first byte of a string may result in unexpected behavior. To prevent this unexpected behavior, Rust panics.
We'll see the differences between Rust's two most commonly used types now:

## `str` vs `String`
Rust has several String types, but the most commonly used are `str` and `String`. 
* `str` is the only String type in the core language, and represents a string slice. Its borrowed form, `&str`, is the most often used. It consists of a reference to UTF-8 encoded data stored elsewhere, therefore we don't own that data. String literals are stored directly in the binary. We can think of `&str` as a closed window: it allows us to see the outside world from our houses, but we can't interact with it.
* `String` is a growable, mutable and owned String type that is stored on the heap. It is part of the standard library. It stores UTF-8 encoded data.

## Operations with Strings.
Rust allows several operations with Strings in Rust. Check out `main.rs` for a quick overview!
