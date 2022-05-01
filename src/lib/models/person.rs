use crate::lib::{
    traits::*,
    enums::Direction,
    enums::Direction::*,
};

#[derive(Clone, Copy)]
pub struct Person {
    pub destination: u64,
}

impl Deliverable for Person {
    fn get_direction(&self, current: u64) -> Direction {
        if self.destination > current {
            return Up
        }
        if self.destination < current {
            return Down
        }
        // Destination == Current
        return Direction::None
    }

    fn is_arrived(&self, current: u64) -> bool {
        match self.get_direction(current) {
            Direction::None => true,
            _ => false
        }
    }

    fn is_going_in(&self, direction: Direction, current: u64) -> bool {
        self.get_direction(current) == direction
    }
}