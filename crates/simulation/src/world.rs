use std::collections::HashMap;

use crate::person::{Person, PersonId};

#[derive(Debug)]
pub struct World {
    pub name: String,
    pub time: SimTime,
    people: HashMap<PersonId, Person>,
}

impl World {
    pub fn new(name: String) -> Self {
        Self { 
            name,
            time: SimTime(0),
            people: HashMap::new(),
        } 
    }

    pub fn tick(&mut self) {
        self.time.0 += 1;
    }

    pub fn population(&self) -> u64 {
        self.people.len() as u64
    }

    pub fn add_person(&mut self, person: Person) {
        self.people.insert(person.id.clone(), person);
    }

    pub fn get_person(&self, id: PersonId) -> Option<&Person> {
        self.people.get(&id)
    }

    pub fn get_person_mut(&mut self, id: PersonId) -> Option<&mut Person> {
        self.people.get_mut(&id)
    }
}

#[derive(Debug, PartialEq)]
pub struct SimTime(u64);

impl SimTime {
    pub fn new(time: u64) -> Self {
        Self(time)
    }
}

pub fn add_founders(world: &mut World) {

    let names = ["Alice", "Bob", "Clara", "Daniel", "Marcus"];
    
    for name in names {
        let person = Person::new(name.to_string(), PersonId::new(world.population() + 1));
        world.add_person(person);
    }
}