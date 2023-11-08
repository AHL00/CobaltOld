use std::time::Duration;

use cobalt::{system::System, AppBuilder};

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

    app.register_system(System::timed(
        "Perf Stats".to_string(),
        |app, delta| {
            println!("FPS: {}, Frame Time: {}", app.perf_stats.fps, app.perf_stats.avg_frame_time);
        },
        Duration::from_millis(1000),
    ));

    let res = app.run();

    println!("App quit with result: {:?}", res);
}
