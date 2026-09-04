mod person;
mod world;

pub use person::PersonId;
pub use world::{SimTime, World};

#[cfg(test)]
mod tests {

    use super::{PersonId, SimTime, World};

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
}
