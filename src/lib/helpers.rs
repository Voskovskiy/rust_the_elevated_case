use rand::Rng;
use rand::rngs::ThreadRng;

#[derive(Clone, Copy)]
pub struct Random {
    pub limit: u64
}

impl Random {
    pub fn number(&self) -> u64 {
        let mut rng: ThreadRng = rand::thread_rng();
        let number: u64 = rng.gen_range(0..self.limit);
        number
    }
    pub fn number_with(&self, limit: u64) -> u64 {
        let mut rng: ThreadRng = rand::thread_rng();
        let number: u64 = rng.gen_range(0..limit);
        number
    }
    
    pub fn capacity(&self) -> usize {
        self.number() as usize
    }
}