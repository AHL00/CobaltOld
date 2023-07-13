use std::cell::RefMut;
use std::rc::Rc;

use crate::core::graphics::*;
use crate::resources::Res;
use glad_gl::gl;
use nalgebra_glm as glm;

// Exports
pub use crate::core::graphics::image::Image;
pub use crate::core::graphics::texture::Texture;

pub struct Sprite {
    pub texture: Option<Res<texture::Texture>>,
    vao: buffer::VertexArray,
    vbo: buffer::VertexBuffer,
    ebo: buffer::IndexBuffer,
}

impl Sprite {
    pub fn new() -> Sprite {
        let mut vbo = buffer::VertexBuffer::new();
        let mut vao = buffer::VertexArray::new();
        let mut ebo = buffer::IndexBuffer::new();

        vao.bind();
        vbo.bind();
        ebo.bind();

        let vertices: [f32; 20] = [
            // positions     // texture coords
            0.5, 0.5, 0.0, 1.0, 1.0, // top right
            0.5, -0.5, 0.0, 1.0, 0.0, // bottom right
            -0.5, -0.5, 0.0, 0.0, 0.0, // bottom left
            -0.5, 0.5, 0.0, 0.0, 1.0, // top left
        ];

        let indices: [u32; 6] = [
            0, 1, 3, // first triangle
            1, 2, 3, // second triangle
        ];

        vbo.set_data(&vertices, buffer::BufferUsage::STATIC_DRAW);
        ebo.set_data(&indices, buffer::BufferUsage::STATIC_DRAW);

        vao.set_layout(&[3, 2], buffer::DataType::FLOAT, false);

        vao.unbind();
        vbo.unbind();
        ebo.unbind();

        Sprite {
            texture: None,
            vao,
            vbo,
            ebo,
        }
    }
}

pub struct Camera2D {
    pub transform: crate::Transform2D,
}

impl Camera2D {
    pub fn new() -> Camera2D {
        Camera2D {
            transform: crate::Transform2D::new(),
        }
    }

    fn get_view_matrix(&self) -> glm::Mat4 {
        let mut view = glm::Mat4::identity();
        view = glm::translate(
            &view,
            &glm::vec3(-self.transform.position.x, -self.transform.position.y, 0.0),
        );
        view = glm::rotate(&view, self.transform.rotation, &glm::vec3(0.0, 0.0, 1.0));
        view
    }

    fn get_projection_matrix(&self, width: f32, height: f32) -> glm::Mat4 {
        let mut projection = glm::ortho(
            -width / 2.0,
            width / 2.0,
            -height / 2.0,
            height / 2.0,
            -1.0,
            1.0,
        );
        projection = glm::scale(
            &projection,
            &glm::vec3(self.transform.scale.x, self.transform.scale.y, 1.0),
        );
        projection
    }
}

macro_rules! load_shader {
    ($vertex:expr, $fragment:expr) => {{
        let vertex_shader = shader::ShaderComponent::new(shader::ShaderType::VERTEX_SHADER);
        vertex_shader.compile(include_str!($vertex));
        if !vertex_shader.get_compile_status() {
            panic!("{}", vertex_shader.get_compile_log());
        }

        let fragment_shader = shader::ShaderComponent::new(shader::ShaderType::FRAGMENT_SHADER);
        fragment_shader.compile(include_str!($fragment));
        if !fragment_shader.get_compile_status() {
            panic!("{}", fragment_shader.get_compile_log());
        }

        let shader_program = shader::ShaderProgram::new();
        shader_program.attach(&vertex_shader);
        shader_program.attach(&fragment_shader);
        shader_program.link();
        if !shader_program.get_link_status() {
            panic!("{}", shader_program.get_link_log());
        } else {
            // get the last word before .vert and after /
            let name = $vertex
                .split('/')
                .last()
                .unwrap()
                .split('.')
                .next()
                .unwrap();
            log::trace!("Compiled shader program: {}", name);
            shader_program
        }
    }};
}

pub struct Renderer {
    pub camera: Camera2D,
    _sprite_shader: shader::ShaderProgram,
    _iters: usize,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            camera: Camera2D::new(),
            _sprite_shader: load_shader!("shaders/sprite.vert", "shaders/sprite.frag"),
            _iters: 0,
        }
    }

    pub fn render(
        &mut self,
        world: &crate::ecs::World,
        resources: &crate::resources::ResourceManager,
    ) {
        self._iters += 1;

        self._sprite_shader.use_program();
        self._sprite_shader.set_uniform_1i("sprite_texture", 0);

        gl_check_error!();

        for (_, (sprite, transform)) in world.query::<(&mut Sprite, &crate::Transform2D)>().iter() {
            // bind the texture
            if sprite.texture.is_none() {
                continue;
            }

            let texture = resources.get(&sprite.texture.unwrap()).unwrap();
            texture.bind();

            // bind the vertex data
            sprite.vao.bind();
            sprite.vbo.bind();
            sprite.ebo.bind();

            // set the transform
            // let model = transform.get_model_matrix();
            // let view = self.camera.transform.get_view_matrix();
            // let projection = self.camera.transform.get_projection_matrix();
            // let mvp = projection * view * model;

            // skip transform and matrices for now
            unsafe {
                gl::DrawElements(
                    gl::TRIANGLES,
                    sprite.ebo.size as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            }

            // check gl errors
            gl_check_error!();
        }
    }
}
