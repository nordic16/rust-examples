use super::Vehicle;

pub struct Bike {
    pub brand: String,
    current_speed: u32,
}

impl Vehicle for Bike {
    fn brake(&mut self, decrement: u32) {
        let decrement = if decrement <= self.current_speed { decrement } else { self.current_speed };
        self.current_speed -= decrement;
    }

    fn stop(&mut self) {
        self.current_speed = 0;
    }

    fn set_speed(&mut self, speed: u32) {
        self.current_speed = speed;
    }

    fn summarize(&self) -> String {
        format!("Brand: {}\nCurrent speed: {}", self.brand, self.current_speed)
    }

    fn new(brand: &String, current_speed: u32) -> Self {
        Bike {brand: brand.to_owned(), current_speed}
    }

    fn default_behaviour(&self) {
        println!("Just finished overriding default_behaviour()");
    }

    fn get_current_speed(&self) -> u32 {
        self.current_speed
    }
}