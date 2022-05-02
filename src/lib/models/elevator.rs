use crate::lib::{
    enums::{
      Direction,
      Direction::*,
    },
    traits::*,
    helpers::Random,
    models::{
        building::Building,
        person::Person,
    }
};

pub struct Elevator {
    started: bool,
    pub steps: u64,
    pub current_floor: u64,
    pub capacity: u64,
    direction: Direction,
    passengers: Vec<Person>,
    building: Building,
}

impl Elevator {
    pub fn new(random: Random) -> Self {
        let capacity = random.number();
        let building = Building::new(random);
        Self {
            started: false,
            current_floor: 0,
            steps: 0,
            capacity,
            direction: Direction::None,
            passengers: Vec::new(),
            building,
        }
    }

    pub fn start(&mut self, floor: u64) {
        if self.started {
            return
        }
        let top_floor = self.building.size() - 1;
        if self.building.size() <= floor {
            self.current_floor = top_floor;
        }
        self.current_floor = floor;
        self.started = true
    }

    pub fn finished(&self) -> bool {
        self.building.is_empty() && self.is_empty()
    }

    pub fn skip_floor(&self) -> bool {
        if self.available_passenger_to_deliver() {
            return false
        }
        if self.is_full()
            || self.building.is_empty_floor_at(self.current_floor)
        {
            return true
        }
        !self.building.has_people_to_deliver_in(self.direction, self.current_floor)
    }

    pub fn go_to_next_floor(&mut self) {
        match self.direction {
            Up => self.go_up(),
            Down => self.go_down(),
            _ => self.check_direction(),
        }
    }

    pub fn load_passengers(&mut self) {
        if self.is_full() {
            return;
        }
        self.sort_asc();
        let limit: u64 = self.capacity - self.size();
        let mut passengers: Vec<Person> = self.building.get_people_to_deliver_from(
            self.direction,
            self.current_floor,
            limit
        );
        if !passengers.is_empty() {
            println!("Loaded {} passenger(s).", passengers.len());
        }
        self.passengers.append(&mut passengers);
    }

    pub fn deliver_passengers(&mut self) -> u64 {
        self.sort_asc();
        let mut passengers_left: u64 = 0;
        let mut passengers_stay: Vec<Person> = Vec::new();
        for passenger in self.passengers.iter() {
            if passenger.is_arrived(self.current_floor) {
                passengers_left += 1;
                continue;
            }
            passengers_stay.push(*passenger);
        }
        if passengers_left > 0 {
            self.passengers = passengers_stay;
        }
        passengers_left
    }

    pub fn is_full(&self) -> bool { return self.size() == self.capacity }

    pub fn people_waiting(&self) -> u64 {
        self.building.people_inside()
    }

    fn available_passenger_to_deliver(&self) -> bool {
        if !self.passengers.is_empty() {
            for person in self.passengers.iter() {
                if person.destination == self.current_floor {
                    return true
                }
            }
        }
        false
    }
}

impl Sizeable for Elevator {
    fn is_empty(&self) -> bool {
        self.passengers.is_empty()
    }
    fn is_not_empty(&self) -> bool {
        !self.passengers.is_empty()
    }
    fn size(&self) -> u64 { self.passengers.len() as u64 }
}

impl Sortable for Elevator {
    fn sort_asc(&mut self) {
        self.passengers.sort_by(|a, b| a.destination.cmp(&b.destination));
        self.passengers.reverse();
    }
}
impl Movable for Elevator {
    fn check_direction(&mut self) {
        if self.direction != Direction::None {
            return
        }
        let people_at_upper_floors = self.building.people_going_in_direction_from(Up, self.current_floor);
        let people_at_lower_floors = self.building.people_going_in_direction_from(Down, self.current_floor);
        if people_at_upper_floors > people_at_lower_floors {
            self.direction = Up
        }
        self.direction = Down;
        self.go_to_next_floor()
    }
    fn change_direction(&mut self) {
        match self.direction {
            Up => {
                self.direction = Down;
                self.go_down();
            },
            Down => {
                self.direction = Up;
                self.go_up();
            },
            _ => {}
        }
    }
    fn go_up(&mut self) {
        if self.current_floor == self.building.last_floor() {
            self.change_direction()
        }
        self.current_floor += 1;
        self.steps += 1
    }
    fn go_down(&mut self) {
        if self.current_floor == 0 {
            self.change_direction()
        }
        self.current_floor -= 1;
        self.steps += 1
    }
}