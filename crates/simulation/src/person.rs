#[derive(Debug, PartialEq)]
pub struct Person {
    pub id: PersonId,
    pub name: String,
}

impl Person {
    pub fn new(name: String, id: PersonId) -> Self {
        Self { id, name }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PersonId(u64);

impl PersonId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}