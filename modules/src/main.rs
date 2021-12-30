/* Rust provides a way for us to organize our code: through modules.
   More about modules: https://doc.rust-lang.org/stable/rust-by-example/mod.html
*/

// In Rust, every file is a module.
// Rust looks for human.rs or human/mod.rs.
// Essentially, you need to declare the existence of a module before being able to use it.
mod human;

// In order to bring one of the public items into scope, use the following syntax:
// The use keyword essentially creates "aliases" for a full path.
// use human::arms; 

// The line above brings the module arms into scope, meaning we now no longer need to type its full path to access its public items.
// Since we want to import not just arms, but head and legs, we'd need to write the statement above 3 times, replacing arms with the respective name.
// To tidy up our code, Rust allows nested imports:
use human::{
    head,
    arms,
    legs
};

// brings human::head, human::arms and human::legs into scope.

fn main() {
    // Had we not specified that we were using head, we'd have to prepend human:: to this statement.
    // Same thing happens to the next examples.
    head::eat();
    head::talk();
    head::think();

    arms::shoot_force_lightning();
    arms::chest_press(45);

    legs::leg_press(70);
    legs::moving(7);

    // This line won't work, as head::feel() is only visible inside human.
    // head::feel();

    // This line won't work, as arms::ignite_lightsaber() is private.
    // A reminder: only a Sith deals in absolutes.
    //arms::ignite_lightsaber();

    // This line won't work, as legs::do_something() is private.
    // legs::do_something();

    // Note that we can still access these 3 private elements indirectly, by calling them from one of the many functions available to us.
    
}
