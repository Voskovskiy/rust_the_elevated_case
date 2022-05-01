use crate::lib::{
    helpers::Random,
    traits::*,
    enums::{
        Direction,
    },
    models::{
        floor::Floor,
        person::Person,
    },
};

pub struct Building {
    floors: Vec<Floor>
}

impl Building {
    pub fn new(random: Random) -> Self {
        let mut floors: Vec<Floor> = Vec::new();
        let number_of_floors: u64;
        let random_number = random.number();
        if random_number < 5 {
            number_of_floors = 10;
        } else {
            number_of_floors = random_number;
        }
        for index in 0..number_of_floors {
            floors.push(Floor::new(index, random, number_of_floors));
        }
        Self {
            floors
        }
    }

    pub fn people_going_in_direction_from(&self, direction: Direction, index: u64) -> u64 {
        let mut qty: u64 = 0;
        for floor in self.floors.iter() {
            qty += floor.people_going_in(direction, index);
        }
        qty
    }

    pub fn has_people_to_deliver_in(&self, direction: Direction, floor: u64) -> bool {
        self.people_going_in_direction_from(direction, floor) != 0
    }

    pub fn get_people_to_deliver_from(&mut self, direction: Direction, index: u64, limit: u64) -> Vec<Person> {
        for floor in self.floors.iter_mut() {
            if floor.number != index {
                continue;
            }
            return floor.load_people(direction, limit);
        }
        Vec::new()
    }

    fn people_at_floor(&self, index: u64) -> u64 {
        for floor in self.floors.iter() {
            if floor.number == index {
                return floor.people_qty();
            }
        }
        0
    }

    pub fn is_empty_floor_at(&self, index: u64) -> bool {
        self.people_at_floor(index) == 0
    }

    pub fn last_floor(&self) -> u64 {
        self.size() - 1
    }

    pub fn people_inside(&self) -> u64 {
        let mut qty: u64 = 0;
        for floor in self.floors.iter() {
            if floor.is_empty() {
                continue;
            }
            qty += floor.people_qty();
        }
        qty
    }
}

impl Sizeable for Building {
    fn is_empty(&self) -> bool {
        let mut iterator = self.floors.iter();
        loop {
            if let Some(floor) = iterator.next() {
                if !floor.is_empty() {
                    return false
                }
                continue
            }
            break
        }
        true
    }
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
    fn size(&self) -> u64 {
        self.floors.len() as u64
    }
}