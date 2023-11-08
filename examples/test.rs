use std::time::Duration;

use cobalt::{system::System, App, AppBuilder};

fn main() {
    let mut app = AppBuilder::new();

    app.register_system(System::timed(
        "Physics".to_string(),
        |app, delta| {
            
        },
        Duration::from_millis(10),
    ));

    let res = app.run();

    println!("App quit with result: {:?}", res);
}
