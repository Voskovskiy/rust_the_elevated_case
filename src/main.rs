use lib::{
    models::elevator::Elevator,
    helpers::Random,
};

use crate::lib::traits::Sizeable;

mod lib;

fn main() {
    version();
    // set default variables
    let random = Random { limit: 100 };
    let mut elevator = Elevator::new(random);
    // set initial floor to start
    elevator.start(random.number());
    // start main loop
    loop {
        // check result conditions
        if elevator.finished() {
            finished(elevator.steps);
            break
        }
        // check status
        status(
            elevator.current_floor,
            elevator.size(),
            elevator.capacity,
            elevator.people_waiting(),
        );
        // check current floor
        if !elevator.skip_floor() {
            // Deliver passengers at current floor
            passengers_arrived(elevator.deliver_passengers());
            // Load passengers from current floor
            elevator.load_passengers();
        }
        // proceed to the next floor
        elevator.go_to_next_floor();
    }
}

fn version() {
    println!("Rust The Elevated Case version 0.1.0")
}

fn status(current_floor: u64, size: u64, capacity: u64, people_inside: u64) {
    print!(
        "Floor: {} [{}/{}]",
        current_floor,
        size,
        capacity,
    );
    if people_inside > 0 {
        println!(" - People waiting: {}", people_inside);
        return
    }
    println!()
}

fn passengers_arrived(qty: u64) {
    if qty == 0 {
        return;
    }
    println!("Passengers arrived: {}", qty)
}

fn finished(steps: u64) {
    println!("Finished in {} steps!", steps)
}
