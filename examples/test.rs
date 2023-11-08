use std::time::Duration;

use cobalt::{system::System, App, AppBuilder};

fn main() {    
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

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
