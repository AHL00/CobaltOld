use std::time::Duration;

use cobalt::{system::System, App, AppBuilder};

fn main() {
    let mut app = AppBuilder::new();

    app.register_system(System::once("Graphics".to_string(), |app, delta| {
        println!("Graphics system");
    }));

    app.register_system(System::timed(
        "Physics".to_string(),
        |app, delta| {
            println!("Physics system: delta: {:?}", delta);
        },
        Duration::from_millis(1500),
    ));

    app.run();
}
