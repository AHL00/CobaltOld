use ahash::AHashMap;

use crate::{camera, App};


pub struct Scene {
    pub world: hecs::World,
    pub camera: Option<camera::Camera>,
}

impl Scene {


}

pub struct ScenesManager {
    scenes: AHashMap<String, SceneGenerator>,
    current_scene: Option<Scene>,
    // This is an workaround to allow the scene generator to access the app.
    // If not, the user will have to call app.scenes.load("test", app) instead of app.scenes.load("test").
    // This causes a double mutable borrow, which is not allowed.
    // This will be set by AppBuilder.run() before any systems are run
    pub(crate) app_ref: *mut App,
}

impl ScenesManager {
    pub(crate) fn new() -> Self {
        Self {
            scenes: AHashMap::new(),
            current_scene: None,
            app_ref: std::ptr::null_mut(),
        }
    }

    pub fn current(&self) -> Option<&Scene> {
        self.current_scene.as_ref()
    }

    pub fn current_mut(&mut self) -> Option<&mut Scene> {
        self.current_scene.as_mut()
    }

    pub fn load<S: Into<String>>(&mut self, name: S) -> anyhow::Result<()> {
        let scene_gen = self.scenes.get(&name.into()).ok_or(anyhow::anyhow!("Scene not found."))?;

        self.current_scene = Some(scene_gen.generate(unsafe { &mut *self.app_ref }));
        Ok(())
    }

    pub fn add<S: Into<String>>(&mut self, name: S, scene_gen: SceneGenerator) {
        self.scenes.insert(name.into(), scene_gen);
    }
}


/// This struct allows the scene manager to load a scene.
/// It stores a function that is called when the scene is loaded.
pub struct SceneGenerator {
    pub(crate) on_load: Option<Box<dyn Fn(&mut Scene, &mut App)>>,

}

impl SceneGenerator {
    /// This function is called every time the scene is loaded.
    /// When this is called, the world and camera will be empty.
    /// Fill the world with entities and add a camera to the scene.
    pub fn on_load(&mut self, f: impl Fn(&mut Scene, &mut App) + 'static) {
        self.on_load = Some(Box::new(f));
    }

    /// Create a new scene generator.
    /// f is called when the scene is loaded.
    pub fn new(f: impl Fn(&mut Scene, &mut App) + 'static) -> Self {
        Self {
            on_load: Some(Box::new(f)),
        }
    }

    pub(crate) fn generate(&self, app: &mut App) -> Scene {
        let mut scene = Scene {
            world: hecs::World::new(),
            camera: None,
        };

        if let Some(f) = &self.on_load {
            f(&mut scene, app);
        }

        scene
    }

}