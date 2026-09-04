mod world;

pub use world::{World, SimTime};

#[cfg(test)]
mod tests {

    use super::{World, SimTime};

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
}
