pub mod car;
pub mod bike;

pub trait Vehicle {
    fn brake(&mut self, decrement: u32);
    fn stop(&mut self);
    fn set_speed(&mut self, speed: u32);
    fn summarize(&self) -> String;
    
    // This function, unlike the previous ones, won't have to be defined in every time Vehicle is implemented by.
    // If it is defined, this definition will be overridden.
    fn default_behaviour(&self) {
        println!("Let's gooooo!");
    }    

    fn new(brand: &String, current_speed: u32) -> Self;
}