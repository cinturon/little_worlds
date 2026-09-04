#[derive(Debug)]
pub struct World {
    pub name: String,
    pub time: SimTime,
}

#[derive(Debug, PartialEq)]
pub struct SimTime(u64);

impl SimTime {
    pub fn new(time: u64) -> Self {
        Self(time)
    }
}

impl World {
    pub fn new(name: String) -> Self {
        Self { 
            name,
            time: SimTime(0),
        } 
    }
}