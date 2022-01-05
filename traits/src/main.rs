// More on traits: https://doc.rust-lang.org/stable/book/ch10-02-traits.html

mod vehicle;

use vehicle::{
    Vehicle,
    car::Car,
    bike::Bike,
};


fn main() {
    let my_car = Car::new(&String::from("Maserati"), 200);
    let my_bike = Bike::new(&String::from("FELT"), 20); 

    println!("{}\n", my_car.summarize());
    println!("{}\n", my_bike.summarize());

    // Notice the difference between the output of the next two lines.
    my_car.default_behaviour();
    my_bike.default_behaviour();


    if my_car.faster(&my_bike) {
        println!("car is faster.");
    }

}


/* 
    This doesn't work: impl Trait can only be used with functions that return a single type.

fn returns_vehicle(car: bool) -> impl Vehicle {
    if car {
        Car::new(&String::from("Ford"), 20)
    } else {
        Bike::new(&String::from("I don't know anything about bikes lmao"), 10)
    }
}
*/