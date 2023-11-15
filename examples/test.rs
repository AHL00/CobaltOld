use std::time::Duration;

use cobalt::{system::System, AppBuilder, assets::Asset, renderer::renderables::{test_triangle::TestTriangle, quad::Quad, rect::Rect}, texture::Texture};

struct GameState {
    counter: u32,
    asset: Asset<String>,
}

struct Pointer<T> {
    pointer: *mut T,
}

impl<T> Pointer<T> {
    fn new(t: &mut T) -> Self {
        Self {
            pointer: t as *mut T,
        }
    }

    const fn null() -> Self {
        Self {
            pointer: std::ptr::null_mut(),
        }
    }

    unsafe fn as_mut(&self) -> &mut T {
        &mut *self.pointer
    }
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
            }).expect("Failed to create resource.");

            app.world.spawn((1u32, "test".to_string()));
            app.world.spawn((TestTriangle::new(), ));
            // app.world.spawn((Quad::new(&app), ));

            let mut test_texture = app.assets.create_asset(Texture::new(&app.window, include_bytes!("texture.png"))).expect("Failed to create asset.");
            
            app.world.spawn((Rect::with_texture(&app, test_texture.clone()), ));
        },
    )); 

    app.register_system(System::timed("Res test", |app, delta| {
        let res = app.resources.get_resource_mut::<GameState>().unwrap();
        res.counter += 1;

        // Create test asset
        let ass = app.assets.create_asset("test".to_string()).expect("Failed to create asset.");

        let ass2 = ass.clone();

        res.asset = ass2;

        println!("Asset: {}", *ass);

        for (id, (i, string)) in app.world.query_mut::<(&mut u32, &String)>() {
            *i += 1;

            println!("Counter: {}, str: {}", i, string);
        }
    }, Duration::from_millis(1000)));

    let res = app.run();

    println!("App quit with result: {:?}", res);
}
