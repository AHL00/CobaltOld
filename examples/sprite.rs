use cobalt::{ecs, renderer::{primitives::Rect}};

fn main() {
    // check if in debug mode
    #[allow(unused_assignments)]
    let log_lvl = "info".to_string();
    #[cfg(debug_assertions)]
    {
        //log_lvl = "debug".to_string();
    }

    let yaml_config = include_str!("./assets/log_cfg.yaml");

    // find "<level>", replace with log_lvl
    let yaml_config = yaml_config.replace("<level>", &format!("{:?}", log_lvl));

    // Initialize log4rs
    let config = serde_yaml::from_str(yaml_config.as_str()).unwrap();
    log4rs::init_raw_config(config).unwrap();

    let mut app = cobalt::App::new();

    let parent = app.ecs.world.spawn((cobalt::Transform::new(),));
    let _ = app
        .ecs.
        world.spawn((cobalt::Transform::new(), ecs::Parent::new(parent)));
    
    // iterate and fill transforms with random data
    let mut starting_offset = 0.0;
    for (_, transform) in app.ecs.world.query::<&mut cobalt::Transform>().iter() {
        starting_offset += 100.0;
        transform.translate(starting_offset, starting_offset);
    }

    app.run();
}
