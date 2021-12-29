/* Rust provides a way for us to organize our code: through modules.
   More about modules: https://doc.rust-lang.org/stable/rust-by-example/mod.html
*/

// In Rust, every file is a module.
// Rust tries to find human.rs or human/mod.rs.
// Essentially, you need to declara the existence of a module before being able to use it.
mod human;

// The use keyword essentially creates "aliases" for a full path.
// Because we declared we're using these three modules, we no longer need to use human when accessing items inside these modules.
use human::{
    head,
    arms,
    legs
};

fn main() {
    // Had we not specified that we were using head, we'd have to prepend human:: to this statement.
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
