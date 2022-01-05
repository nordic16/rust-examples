use super::Vehicle;

pub struct Car {
    pub brand: String,
    current_speed: u32,
}


impl Vehicle for Car {
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
        Car { brand: brand.to_owned(), current_speed: current_speed }
    }

   
    
}