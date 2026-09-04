mod person;
mod world;

pub use person::{Person, PersonId};
pub use world::{SimTime, World};

#[cfg(test)]
mod tests {

    use super::{Person, PersonId, SimTime, World};

    #[test]
    fn test_world() {
        let world = World::new("Test World".to_string());
        assert_eq!(world.name, "Test World");
        assert_eq!(world.time, SimTime::new(0));
    }

    #[test]
    fn test_tick() {
        let mut world = World::new("Test World".to_string());
        world.tick();
        assert_eq!(world.time, SimTime::new(1));
        world.tick();
        assert_eq!(world.time, SimTime::new(2));
    }

    #[test]
    fn test_person_id() {
        let person_id_1 = PersonId::new(1);
        let also_person_id_1 = PersonId::new(1);
        let person_id_2 = PersonId::new(2);

        assert_eq!(person_id_1, also_person_id_1);
        assert_ne!(person_id_1, person_id_2);
    }

    #[test]
    fn two_people_can_share_a_name() {
        let name = "Alice";
        let first = PersonId::new(1);
        let second = PersonId::new(2);

        assert_eq!(name, "Alice");
        assert_ne!(first, second);
    }

    #[test]
    fn test_person() {
        let person = Person::new("Alice".to_string(), PersonId::new(1));
        assert_eq!(person.name, "Alice");
        assert_eq!(person.id, PersonId::new(1));
    }

    #[test]
    fn test_add_person() {
        let mut world = World::new("Test World".to_string());
        let person = Person::new("Alice".to_string(), PersonId::new(1));
        world.add_person(person);
        assert_eq!(world.population(), 1);
        let person2 = Person::new("Bob".to_string(), PersonId::new(2));
        world.add_person(person2);
        assert_eq!(world.population(), 2);
    }
}
