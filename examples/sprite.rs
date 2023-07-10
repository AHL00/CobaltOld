use cobalt::{renderer, ecs};

fn main() {
    let mut app = cobalt::App::new();
    let parent = app.ecs.spawn((renderer::Sprite::new(), cobalt::Transform2D::new()));
    let child = app.ecs.spawn((renderer::Sprite::new(), cobalt::Transform2D::new(), ecs::Parent::new(parent)));

    // iterate and fill transforms with random data
    let mut starting_offset = 0.0;
    for (_, transform) in app.ecs.query::<&mut cobalt::Transform2D>().iter() {
        starting_offset += 10.0;
        transform.translate(starting_offset, 0.0);
    }

    // fill the sprites with a texture
    let logo = renderer::Image::from_file("examples/assets/logo.png").expect("Failed to load image from file");
    for (_, sprite) in app.ecs.query::<&mut renderer::Sprite>().iter() {
        sprite.load_texture_from_image(&logo);
    }

    app.run();
}
