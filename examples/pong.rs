use std::time::{Duration, Instant};

use cobalt::{
    camera::Projection, renderer_2d::renderables::Rect, system::System, transform::Transform, App,
    AppBuilder,
};
use hecs::Entity;
use ultraviolet::{Vec2, Vec3};

struct GameState {
    left_paddle: Option<Entity>,
    right_paddle: Option<Entity>,
    ball: Option<Entity>,
    score: (u32, u32),
    last_winner: i32,
    last_scored_time: Option<Instant>,
    win_score: u32,
}

impl GameState {}

struct Paddle {
    up_key: cobalt::input::Key,
    down_key: cobalt::input::Key,
    speed: f32,
    moving: i32,
}

struct Ball {
    velocity: Vec2,
}

const BALL_SPEED: f32 = 100.0;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut builder = AppBuilder::new().with_renderer(Box::new(cobalt::Renderer2D::new()));

    builder.register_system(System::startup("Startup", |app, delta| {
        app.window.winit_win.set_resizable(false);
        app.window
            .winit_win
            .set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));

        app.window.winit_win.set_title("Pong");

        app.resources
            .create_resource(GameState {
                left_paddle: None,
                right_paddle: None,
                ball: None,
                score: (0, 0),
                last_winner: 0,
                last_scored_time: None,
                win_score: 5,
            })
            .expect("Failed to create resource.");

        app.scenes.add(
            "scored",
            cobalt::scene::SceneGenerator::new(|scene, app| {
                let resolution = app.window.winit_win.inner_size();

                scene.camera = Some(cobalt::camera::Camera::new(
                    Transform::new(
                        Vec3::new(0.0, 0.0, 1.0),
                        Vec3::new(0.0, 0.0, 180f32.to_radians()),
                        Vec3::new(1.0, 1.0, 1.0),
                    ),
                    Projection::Orthographic {
                        aspect: resolution.width as f32 / resolution.height as f32,
                        height: 100.0,
                        near: -5.0,
                        far: 5.0,
                    },
                    &app.window,
                ));
            }),
        );

        app.scenes.add(
            "pong",
            cobalt::scene::SceneGenerator::new(|scene, app| {
                let resolution = app.window.winit_win.inner_size();

                scene.camera = Some(cobalt::camera::Camera::new(
                    Transform::new(
                        Vec3::new(0.0, 0.0, 1.0),
                        Vec3::new(0.0, 0.0, 180f32.to_radians()),
                        Vec3::new(1.0, 1.0, 1.0),
                    ),
                    Projection::Orthographic {
                        aspect: resolution.width as f32 / resolution.height as f32,
                        height: 100.0,
                        near: -5.0,
                        far: 5.0,
                    },
                    &app.window,
                ));

                let viewport_size = scene.camera.as_ref().unwrap().viewport_size();

                let left_paddle = scene.world.spawn((
                    Transform::new(
                        Vec3::new(-viewport_size.0 / 2.0 + 10.0, 0.0, 0.0),
                        Vec3::zero(),
                        Vec3::new(1.5, 15.0, 1.0),
                    ),
                    Paddle {
                        up_key: cobalt::input::Key::KeyW,
                        down_key: cobalt::input::Key::KeyS,
                        speed: 75.0,
                        moving: 0,
                    },
                    Rect::new((1.0, 1.0, 1.0, 1.0)),
                ));

                let right_paddle = scene.world.spawn((
                    Transform::new(
                        Vec3::new(viewport_size.0 / 2.0 - 10.0, 0.0, 0.0),
                        Vec3::zero(),
                        Vec3::new(1.5, 15.0, 1.0),
                    ),
                    Paddle {
                        up_key: cobalt::input::Key::ArrowUp,
                        down_key: cobalt::input::Key::ArrowDown,
                        speed: 75.0,
                        moving: 0,
                    },
                    Rect::new((1.0, 1.0, 1.0, 1.0)),
                ));

                let initial_velocity = if app
                    .resources
                    .get_resource::<GameState>()
                    .unwrap()
                    .last_winner
                    == 1
                {
                    Vec2::new(-BALL_SPEED, 0.0)
                } else if app
                    .resources
                    .get_resource::<GameState>()
                    .unwrap()
                    .last_winner
                    == -1
                {
                    Vec2::new(BALL_SPEED, 0.0)
                } else {
                    Vec2::new(BALL_SPEED, 0.0)
                };

                let ball = scene.world.spawn((
                    Transform::new(
                        Vec3::new(0.0, 0.0, 0.0),
                        Vec3::zero(),
                        Vec3::new(3.0, 3.0, 1.0),
                    ),
                    Ball {
                        velocity: initial_velocity,
                    },
                    Rect::new((1.0, 1.0, 1.0, 1.0)),
                ));

                let game = app.resources.get_resource_mut::<GameState>().unwrap();

                game.left_paddle = Some(left_paddle);

                game.right_paddle = Some(right_paddle);

                game.ball = Some(ball);

                // Center dotted line
                for i in -5..=5 {
                    scene.world.spawn((
                        Transform::new(
                            Vec3::new(0.0, i as f32 * 10.0, 0.0),
                            Vec3::zero(),
                            Vec3::new(1.0, 4.0, 1.0),
                        ),
                        Rect::new((1.0, 1.0, 1.0, 0.025)),
                    ));
                }
            }),
        );

        app.scenes.load("pong").expect("Failed to load scene.");
    }));

    builder.register_system(System::timed(
        "Scored",
        Some("scored"),
        |app, delta| {
            let game = app.resources.get_resource_mut::<GameState>().unwrap();

            if game.last_scored_time.is_none() {
                game.last_scored_time = Some(Instant::now());
            }

            if game.last_scored_time.unwrap().elapsed().as_secs_f32() > 2.0 {
                game.last_scored_time = None;
                app.scenes.load("pong").expect("Failed to load scene.");
            }
        },
        Duration::from_millis(10),
    ));

    builder.register_system(System::timed(
        "Ball movement",
        Some("pong"),
        |app, delta| {
            let game = app.resources.get_resource_mut::<GameState>().unwrap();
            let viewport_size = app
                .scenes
                .current_scene()
                .unwrap()
                .camera
                .as_ref()
                .unwrap()
                .viewport_size();

            let ball = game.ball.unwrap();

            let world = &mut app.scenes.current_scene_mut().unwrap().world;
            let mut scored = false;

            {
                let (ball, ball_transform) = world
                    .query_one_mut::<(&mut Ball, &mut Transform)>(ball)
                    .unwrap();

                ball_transform.position_mut().x += ball.velocity.x * delta.as_secs_f32();
                ball_transform.position_mut().y += ball.velocity.y * delta.as_secs_f32();

                if ball_transform.position().x > viewport_size.0 / 2.0 {
                    game.last_winner = -1;
                    scored = true;
                }

                if ball_transform.position().x < -viewport_size.0 / 2.0 {
                    game.last_winner = 1;
                    scored = true;
                }
            }

            // Collision with paddles
            let mut bounce_ball_paddle = false;
            let mut bounce_ball_wall = false;
            let mut paddle_moving = 0;

            for (entity, (paddle, transform)) in world.query::<(&Paddle, &Transform)>().iter() {
                let paddle_size = transform.scale().y / 2.0;
                let paddle_position = transform.position().y;

                let ball_transform = world.get::<&Transform>(ball).unwrap();

                let ball_size = ball_transform.scale().x / 2.0;

                let ball_position = ball_transform.position();

                if ball_position.y + ball_size >= paddle_position - paddle_size
                    && ball_position.y - ball_size <= paddle_position + paddle_size
                {
                    if ball_position.x + ball_size >= transform.position().x - transform.scale().x
                        && ball_position.x - ball_size <= transform.position().x + transform.scale().x
                    {
                        bounce_ball_paddle = true;
                        paddle_moving = paddle.moving;
                    }
                }
            }

            // Wall collision
            {
                let ball_transform = world.get::<&Transform>(ball).unwrap();

                let ball_size = ball_transform.scale().x / 2.0;
                let ball_position = ball_transform.position();

                if ball_position.y + ball_size <= -viewport_size.1 / 2.0
                    || ball_position.y - ball_size >= viewport_size.1 / 2.0
                {
                    bounce_ball_wall = true;
                }

            }

            if bounce_ball_wall {
                let (ball, ball_transform) = world
                    .query_one_mut::<(&mut Ball, &mut Transform)>(ball)
                    .unwrap();

                ball.velocity.y *= -1.0 ;
                ball_transform.position_mut().y += ball.velocity.y * delta.as_secs_f32();

                // Normalize
                ball.velocity = ball.velocity.normalized() * BALL_SPEED;
            }

            if bounce_ball_paddle {
                let (ball, ball_transform) = world
                    .query_one_mut::<(&mut Ball, &mut Transform)>(ball)
                    .unwrap();

                ball.velocity.x *= -1.0;
                ball.velocity.y += paddle_moving as f32 * 35.0;

                if paddle_moving == 0 {
                    // Reduce vertical velocity
                    ball.velocity.y *= 0.5;
                }

                ball_transform.position_mut().x += ball.velocity.x * delta.as_secs_f32();

                // Normalize
                ball.velocity = ball.velocity.normalized() * BALL_SPEED;
            }

            if scored {
                app.scenes.load("scored").expect("Failed to load scene.");
            }
        },
        Duration::from_millis(10),
    ));

    builder.register_system(System::timed(
        "Paddle movement",
        Some("pong"),
        |app, delta| {
            for (entity, (paddle, transform)) in app
                .scenes
                .current_scene_mut()
                .unwrap()
                .world
                .query_mut::<(&mut Paddle, &mut Transform)>()
            {
                let mut direction = 0.0;

                paddle.moving = 0;

                if app.input.is_key_down(paddle.up_key) {
                    direction += 1.0;
                    paddle.moving = 1;
                }

                if app.input.is_key_down(paddle.down_key) {
                    direction -= 1.0;
                    paddle.moving = -1;
                }

                *transform.position_mut() = *transform.position()
                    + Vec3::new(0.0, direction * paddle.speed * delta.as_secs_f32(), 0.0);

                let distance_limit = 50.0 - transform.scale().y / 2.0;

                if transform.position().y > distance_limit {
                    *transform.position_mut() = Vec3::new(
                        transform.position().x,
                        distance_limit,
                        transform.position().z,
                    );
                }

                if transform.position().y < -distance_limit {
                    *transform.position_mut() = Vec3::new(
                        transform.position().x,
                        -distance_limit,
                        transform.position().z,
                    );
                }
            }
        },
        Duration::from_millis(10),
    ));

    let res = builder.run();

    if let Err(e) = res {
        log::error!("Error: {}", e);
    }
}
