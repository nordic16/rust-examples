/* 
  This module is actually human::head, as this file is itself a module.
  */

pub mod head {
    /* As nested modules are private (can't be used outside the module scope) by default, we need to make them public (can be accessed outside the module scope)
     through the pub keyword. 
     
     NOTE: Despite being public, items inside public modules are also private by default.
     */

     pub fn think() {
         println!("I'm sentient! Let's goo!!!!");
     }

     // This function will only be accessible inside human.rs. The super keyword refers to the parent module (human).
     pub(super) fn feel() {
         println!("I can feel your anger... It gives you focus, makes you stronger!");
     }

     pub fn talk() {
        println!("YOU WERE THE CHOSEN ONE! IT WAS SAID THAT YOU WOULD DESTROY THE SITH, NOT JOIN THEM! BRING BALANCE TO THE FORCE, NOT LEAVE IT IN DARKNESS!");
     }

     // Will not be accessible outside head.
     fn personal() {
         println!("I'm losing my mind!");
     }

     // Visible within the current crate
     pub(crate) fn eat() {
         println!("That food was delicious.");
     }
}

pub mod arms {
    pub fn shoot_force_lightning() {
        println!("Unlimited power!!!!!!!!!!");
    }

    pub fn chest_press(weight: i16) {
        println!("Just chest pressed {}kg!!!", weight);
    }

    fn ignite_lightsaber() {
        println!("Only a Sith deals in absolutes... I will do what I must.");
    }
}

pub mod legs {
    pub fn moving(speed: i16) {
        if speed < 9 {
            println!("I'm walking!");
        
        } else {
            println!("I'm running!");
        }
    }

    pub fn leg_press(weight: i16) {
        println!("Just leg pressed {}kg!", weight);
    }

    // Ran out of ideas lmao
    fn do_something(s: &String) {
        println!("{}", s);
    }
}