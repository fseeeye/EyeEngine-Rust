use legion::*;
use eyengine::{Application, Transform};

struct SimpleApp;

impl Application for SimpleApp {
    fn update(&self) {}
}

fn main() {
    let app = SimpleApp {};

    // Create a world to store our entities
    let mut world = World::default();
    let _entity = world.push(((), vec![Transform::new()]));

    // Start Application window event loop
    app.start();
}