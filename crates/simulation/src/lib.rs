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
}
