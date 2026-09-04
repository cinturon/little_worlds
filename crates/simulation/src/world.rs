#[derive(Debug)]
pub struct World {
    pub name: String,
    pub time: SimTime,
}

impl World {
    pub fn new(name: String) -> Self {
        Self { 
            name,
            time: SimTime(0),
        } 
    }

    pub fn tick(&mut self) {
        self.time.0 += 1;
    }
}

#[derive(Debug, PartialEq)]
pub struct SimTime(u64);

impl SimTime {
    pub fn new(time: u64) -> Self {
        Self(time)
    }
}