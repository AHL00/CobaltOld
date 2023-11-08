use std::time::Duration;

use cobalt::{system::System, AppBuilder, assets::Asset};

struct GameState {
    counter: u32,
    asset: Asset<String>,
}

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

    app.register_system(System::startup(
        "Res Asset test".to_string(),
        |app, delta| {
            let asset = app.assets.create_asset("test".to_string()).expect("Failed to create asset.");
            
            app.resources.create_resource(GameState {
                counter: 0,
                asset,
            });
        },
    ));

    app.register_system(System::timed("Res test", |app, delta| {
        let res = app.resources.get_resource_mut::<GameState>().unwrap();
        res.counter += 1;
        println!("Counter: {}", res.counter);

        let test_str = res.asset.as_mut();

        test_str.push_str("a");

        println!("Asset: {}", test_str);

    }, Duration::from_millis(1000)));

    let res = app.run();

    println!("App quit with result: {:?}", res);
}
