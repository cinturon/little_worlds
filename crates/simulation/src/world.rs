use crate::person::Person;

#[derive(Debug)]
pub struct World {
    pub name: String,
    pub time: SimTime,
    people: Vec<Person>,
}

impl World {
    pub fn new(name: String) -> Self {
        Self { 
            name,
            time: SimTime(0),
            people: Vec::new(),
        } 
    }

    pub fn tick(&mut self) {
        self.time.0 += 1;
    }

    pub fn population(&self) -> usize {
        self.people.len()
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.push(person);
    }
}

#[derive(Debug, PartialEq)]
pub struct SimTime(u64);

impl SimTime {
    pub fn new(time: u64) -> Self {
        Self(time)
    }
}