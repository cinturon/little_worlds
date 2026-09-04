

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct PersonId(u64);

impl PersonId {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}