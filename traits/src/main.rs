mod vehicle;

use vehicle::{
    car::Car,
    bike::Bike,
};

use crate::vehicle::Vehicle;

fn main() {
    let my_car = Car::new(&String::from("Maserati"), 200);
    let my_bike = Bike::new(&String::from("FELT"), 20); 

    println!("{}\n", my_car.summarize());
    println!("{}\n", my_bike.summarize());

    // Notice the difference between the output of the next two lines.
    my_car.default_behaviour();
    my_bike.default_behaviour();
}
