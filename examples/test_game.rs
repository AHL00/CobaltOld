use std::time::Duration;

use cobalt::{
    physics_2d::rigidbody::Rigidbody2D, renderer_2d::renderables::{Sprite, TranslucentSprite, Text},
    system::System, texture::Texture, transform::Transform, AppBuilder,
};
use hecs::Entity;
use ultraviolet::Vec3;

struct Game {
    fps_text: Option<Entity>,
    start_time: std::time::Instant,
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut app = AppBuilder::new()
        .with_renderer(Box::new(cobalt::Renderer2D::new()))
        .with_physics(Box::new(cobalt::Physics2D::new()));
    
    app.register_system(System::event_callback(
        "Window Resize",
        |app, delta| {
            let size = app.window.winit_win.inner_size();
            
            // Change camera aspect ratio
            if let Some(camera) = app.scenes.current_mut().unwrap().camera.as_mut() {
                if let cobalt::camera::Projection::Orthographic { aspect, .. } = &mut camera.projection {
                    *aspect = size.width as f32 / size.height as f32;
                }
            }
        },
        cobalt::system::EventCallbackType::WindowResize,
    ));

    app.register_system(System::startup("Add Scenes", |app, delta| {
        app.perf_stats.collection_duration = Duration::from_millis(100);

        app.scenes.add(
            "test",
            cobalt::scene::SceneGenerator::new(|scene, app| {
                let mut game = Game {
                    fps_text: None,
                    start_time: std::time::Instant::now(),
                };

                let sprite_texture = app
                    .assets
                    .create_asset(Texture::load(
                        &app.window,
                        include_bytes!("../images/logo.png"),
                    ))
                    .expect("Failed to create asset.");

                    let bg_texture = app
                    .assets
                    .create_asset(Texture::load(
                        &app.window,
                        include_bytes!("texture.png"),
                    ))
                    .expect("Failed to create asset.");

                let translucent_texture = app
                    .assets
                    .create_asset(Texture::load(
                        &app.window,
                        include_bytes!("translucent.png"),
                    ))
                    .expect("Failed to create asset.");                
                
                scene.world.spawn((
                    Sprite::new(&app, sprite_texture.clone()),
                    Transform::new(
                        Vec3::new(3.0, 0.0, 3.5),
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(3.0, 3.0, 1.0),
                    ),
                ));

                scene.world.spawn((
                    Sprite::new(&app, sprite_texture.clone()),
                    Transform::new(
                        Vec3::new(-2.0, 25.0, 3.5),
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(1.0, 1.0, 1.0),
                    ),
                    Rigidbody2D::new(),
                ));

                game.fps_text = Some(scene.world.spawn((
                    Text::new("Cobalt Engine. Lorem ipsum dolor sit amet.", (800.0, 600.0), 20.0, 24.0),
                    Transform::new(
                        Vec3::new(0.0, 0.0, 2.0),
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(1.0, 1.0, 1.0),
                    ),
                    Rigidbody2D::new(),
                )));

                scene.world.spawn((
                    TranslucentSprite::new(&app, translucent_texture.clone()),
                    Transform::new(
                        Vec3::new(0.0, 4.0, 3.0),
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(2.0, 2.0, 1.0),
                    ),
                ));

                scene.world.spawn((
                    TranslucentSprite::new(&app, translucent_texture.clone()),
                    Transform::new(
                        Vec3::new(0.0, 4.0, 4.0),
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(3.0, 1.5, 1.0),
                    ),
                ));
                
                scene.world.spawn((
                    Sprite::new(&app, bg_texture.clone()),
                    Transform::new(
                        Vec3::new(0.0, 0.0, -1.0),
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::new(15.0 * (16.0 / 9.0), 15.0, 1.0),
                    ),
                ));

                // Set camera
                scene.camera = Some(cobalt::camera::Camera::new(
                    Transform::new(
                        Vec3::new(0.0, 0.0, 5.0),
                        Vec3::new(0.0, 0.0, 180_f32.to_radians()),
                        Vec3::new(1.0, 1.0, 1.0),
                    ),
                    cobalt::camera::Projection::Orthographic {
                        aspect: 16.0 / 9.0,
                        height: 15.0,
                        near: -0.0,
                        far: 10.0,
                    },
                    &app.window,
                ));

                app.resources.create_resource(game);
            }),
        );

        app.scenes.load("test").expect("Failed to load scene.");
    }));

    app.register_system(System::timed(
        "Input".to_string(),
        |app, delta| {
            for (id, (transform, rigidbody)) in app
                .scenes
                .current_mut()
                .unwrap()
                .world
                .query_mut::<(&mut Transform, &Rigidbody2D)>()
            {
                if app.input.is_key_down(cobalt::input::Key::KeyW) {
                    transform.position_mut().y += 10.0 * delta.as_secs_f32();
                }

                if app.input.is_key_down(cobalt::input::Key::KeyS) {
                    transform.position_mut().y -= 10.0 * delta.as_secs_f32();
                }

                if app.input.is_key_down(cobalt::input::Key::KeyA) {
                    transform.position_mut().x -= 10.0 * delta.as_secs_f32();
                }

                if app.input.is_key_down(cobalt::input::Key::KeyD) {
                    transform.position_mut().x += 10.0 * delta.as_secs_f32();
                }
            }

            if app.input.is_key_down(cobalt::input::Key::ArrowRight) {
                if let Some(camera) = app.scenes.current_mut().unwrap().camera.as_mut() {
                    camera.transform.position_mut().x += 10.0 * delta.as_secs_f32();
                }
            }

            if app.input.is_key_down(cobalt::input::Key::ArrowLeft) {
                if let Some(camera) = app.scenes.current_mut().unwrap().camera.as_mut() {
                    camera.transform.position_mut().x -= 10.0 * delta.as_secs_f32();
                }
            }

            if app.input.is_key_down(cobalt::input::Key::ArrowUp) {
                if let Some(camera) = app.scenes.current_mut().unwrap().camera.as_mut() {
                    camera.transform.position_mut().y += 10.0 * delta.as_secs_f32();
                }
            }

            if app.input.is_key_down(cobalt::input::Key::ArrowDown) {
                if let Some(camera) = app.scenes.current_mut().unwrap().camera.as_mut() {
                    camera.transform.position_mut().y -= 10.0 * delta.as_secs_f32();
                }
            }

            if app.input.is_key_clicked(cobalt::input::Key::Space) {
                // Toggle rigidbody
                for (id, rigidbody) in app
                    .scenes
                    .current_mut()
                    .unwrap()
                    .world
                    .query_mut::<&mut Rigidbody2D>()
                {
                    rigidbody.enabled = !rigidbody.enabled;
                }
            }

            if app.input.is_key_clicked(cobalt::input::Key::KeyR) {
                // Reset position
                for (id, (transform, rigidbody)) in app
                    .scenes
                    .current_mut()
                    .unwrap()
                    .world
                    .query_mut::<(&mut Transform, &mut Rigidbody2D)>()
                {
                    *transform.position_mut() = Vec3::zero();
                    rigidbody.reset();
                }
            }
        },
        Duration::from_millis(5),
    ));

    app.register_system(System::timed(
        "Debug".to_string(),
        |app, delta| {
            if let Some(game) = app.resources.get_resource::<Game>() {
                if let Some(fps_text_entity) = game.fps_text {
                    if let Ok(text) = app
                        .scenes
                        .current_mut()
                        .unwrap()
                        .world
                        .query_one_mut::<&mut Text>(fps_text_entity)
                    {
                        text.set_text(format!("FPS: {}\nFrame Time: {:.2?}\nTime: {:.2?}\n", app.perf_stats.fps, app.perf_stats.avg_frame_time, game.start_time.elapsed()));
                    }
                }
            }
        },
        Duration::from_millis(100),
    ));

    let res = app.run();

    println!("App quit with result: {:?}", res);
}
