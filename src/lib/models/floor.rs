use crate::lib::{
    helpers::Random,
    traits::*,
    enums::{
      Direction,
      Direction::*,
    },
    models::person::Person,
};

pub struct Floor {
    pub number: u64,
    people: Vec<Person>,
}

impl Floor {
    pub fn new(number: u64, generator: Random, top_floor: u64) -> Self {
        let mut destinations: Vec<u64> = Vec::new();
        for _ in 0..generator.capacity() {
            let destination = generator.number_with(top_floor);
            if destination == number {
                continue;
            }
            destinations.push(destination);
        }
        let mut people: Vec<Person> = Vec::new();
        for destination in destinations {
            let person = Person { destination };
            people.push(person)
        }
        Self { number, people }
    }

    pub fn people_going_in(&self, direction: Direction, floor: u64) -> u64 {
        if direction == Up && self.number < floor {
            return 0
        }
        if direction == Down && self.number > floor {
            return 0
        }
        let mut qty: u64 = 0;
        for person in self.people.iter() {
            let person_direction = person.get_direction(self.number);
            if person_direction == direction {
                qty += 1;
            }
        }
        qty
    }

    pub fn people_qty(&self) -> u64 {
        self.people.len() as u64
    }

    pub fn load_people(&mut self, direction: Direction, limit: u64) -> Vec<Person> {
        let mut capacity: u64 = limit;
        let mut people_to_transfer: Vec<Person> = Vec::new();
        let mut people_to_stay: Vec<Person> = Vec::new();
        for person in self.people.iter() {
            if capacity == 0 {
                break;
            }
            if person.is_going_in(direction, self.number) {
                capacity -= 1;
                people_to_transfer.push(*person);
                continue;
            }
            people_to_stay.push(*person);
        }
        self.people.clear();
        self.people.append(&mut people_to_stay);
        people_to_transfer
    }
}

impl Sizeable for Floor {
    fn is_empty(&self) -> bool {
        self.people.is_empty()
    }
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
    fn size(&self) -> u64 {
        self.people_qty()
    }
}

impl Sortable for Floor {
    fn sort_asc(&mut self) {
        self.people.sort_by(|a,b| a.destination.cmp(&b.destination));
        self.people.reverse()
    }
}