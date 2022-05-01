use crate::lib::enums::Direction;

pub trait Sizeable {
    fn is_empty(&self) -> bool;
    fn is_not_empty(&self) -> bool;
    fn size(&self) -> u64;
}


pub trait Deliverable {
    fn get_direction(&self, current: u64) -> Direction;
    fn is_arrived(&self, current: u64) -> bool;
    fn is_going_in(&self, direction: Direction, current: u64) -> bool;
}

pub trait Sortable {
    fn sort_asc(&mut self);
}