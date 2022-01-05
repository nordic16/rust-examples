pub mod car;
pub mod bike;

pub trait Vehicle {
    fn brake(&mut self, decrement: u32);
    fn stop(&mut self);
    fn set_speed(&mut self, speed: u32);
    fn get_current_speed(&self) -> u32;
    fn summarize(&self) -> String;
    
    // This function can be used with any object that implements this trait.
    fn faster(&self, other: &impl Vehicle) -> bool {
        self.get_current_speed() > other.get_current_speed()
    }
    
    // This method, unlike the previous ones, won't have to be defined in every time Vehicle is implemented by.
    // If it is defined, this definition will be overridden.
    fn default_behaviour(&self) {
        println!("Let's gooooo!");
    }    

    /*
     Note that we're using Self, not self.
     While Self refers to the type that implements the trait, self refers to the instance
     from where a method is called.
    */
    fn new(brand: &String, current_speed: u32) -> Self;
}