use std::time::Duration;

use cobalt::{system::System, AppBuilder, assets::Asset, texture::Texture, renderer_2d::renderables::Rect, transform::Transform};
use ultraviolet::Vec3;

struct GameState {
    counter: u32,
    asset: Asset<String>,
}

// struct Pointer<T> {
//     pointer: *mut T,
// }

// impl<T> Pointer<T> {
//     fn new(t: &mut T) -> Self {
//         Self {
//             pointer: t as *mut T,
//         }
//     }

//     const fn null() -> Self {
//         Self {
//             pointer: std::ptr::null_mut(),
//         }
//     }

//     unsafe fn as_mut(&self) -> &mut T {
//         &mut *self.pointer
//     }
// }

fn main() {    
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut app = AppBuilder::new()
    .with_renderer(Box::new(cobalt::Renderer2D::new()));

    app.register_system(System::timed(
        "Debug".to_string(),
        |app, delta| {

            // world iterate over all transforms
            let mut obj_pos = Vec3::zero();

            for (id, transform) in app.world.query_mut::<&mut Transform>() {
                if app.input.is_key_down(cobalt::input::Key::KeyW) {
                    transform.position_mut().y += 1.0 * delta.as_secs_f32();
                }

                if app.input.is_key_down(cobalt::input::Key::KeyS) {
                    transform.position_mut().y -= 1.0 * delta.as_secs_f32();
                }

                if app.input.is_key_down(cobalt::input::Key::KeyA) {
                    transform.position_mut().x -= 1.0 * delta.as_secs_f32();
                }

                if app.input.is_key_down(cobalt::input::Key::KeyD) {
                    transform.position_mut().x += 1.0 * delta.as_secs_f32();
                }

                obj_pos = *transform.position();
            };

            // Clear line and go up
            for _ in 0..3 {
                print!("\x1b[1A\x1b[2K");
            }

            println!("FPS: {}, Frame Time: {}", app.perf_stats.fps, app.perf_stats.avg_frame_time);
            println!("Camera Position: {:?}", app.camera.transform.position());
            println!("Object Position: {:?}", obj_pos);

        },
        Duration::from_millis(100),
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

            let test_texture = app.assets.create_asset(Texture::new(&app.window, include_bytes!("texture.png"))).expect("Failed to create asset.");
            
            app.world.spawn((Rect::with_texture(&app, test_texture.clone()), Transform::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(1.0, 1.0, 1.0),
            )));
        },
    )); 

    app.register_system(System::timed("Res test", |app, delta| {
        let res = app.resources.get_resource_mut::<GameState>().unwrap();
        res.counter += 1;

        // Create test asset
        let ass = app.assets.create_asset("test".to_string()).expect("Failed to create asset.");

        let ass2 = ass.clone();

        res.asset = ass2;

        // println!("Asset: {}", *ass);

        for (id, (i, string)) in app.world.query_mut::<(&mut u32, &String)>() {
            *i += 1;

            // println!("Counter: {}, str: {}", i, string);
        }
    }, Duration::from_millis(1000)));

    let res = app.run();

    println!("App quit with result: {:?}", res);
}
