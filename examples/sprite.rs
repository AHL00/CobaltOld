use cobalt::{renderer::{self, Texture}, ecs};

fn main() {
    let mut app = cobalt::App::new();
    let parent = app.ecs.spawn((renderer::Sprite::new(), cobalt::Transform2D::new()));
    let child = app.ecs.spawn((renderer::Sprite::new(), cobalt::Transform2D::new(), ecs::Parent::new(parent)));

    // iterate and fill transforms with random data
    let mut starting_offset = 0.0;
    for (_, transform) in app.ecs.query::<&mut cobalt::Transform2D>().iter() {
        starting_offset += 100.0;
        transform.translate(starting_offset, starting_offset);
    }

    // fill the sprites with a texture
    let logo = renderer::Image::from_file("examples/assets/wood.jpg").expect("Failed to load image from file");
    let texture = renderer::Texture::new();
    texture.set_image(&logo);

    let texture = app.res_mut().add::<Texture>(texture);
    
    for (_, sprite) in app.ecs.query::<&mut renderer::Sprite>().iter() {
        sprite.texture = Some(texture);
    }

    app.run();
}
