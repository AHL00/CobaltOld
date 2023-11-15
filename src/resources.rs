use std::any::{Any, TypeId};

use ahash::AHashMap;

/// A resource is a globally accessible object that can be shared between multiple systems.
/// Every resource has a type and every type can only have one resource.
pub struct Res<T>
{
    value: T,
}

/// A resource manager is a collection of resources.
/// It is used to create and access resources.
pub struct ResourceManager {
    resources: AHashMap<std::any::TypeId, Box<dyn Any>>,
}

impl ResourceManager {
    /// Creates a new resource manager.
    pub(crate) fn new() -> Self {
        Self {
            resources: AHashMap::new(),
        }
    }

    /// Creates a new resource of type `T` with the given value.
    pub fn create_resource<T: 'static>(&mut self, value: T) -> anyhow::Result<()> {
        if self.get_resource::<T>().is_some() {
            return Err(anyhow::anyhow!("Resource of type {} already exists.", std::any::type_name::<T>()));
        }

        let type_id = TypeId::of::<T>();
        self.resources.insert(type_id, Box::new(Res { value }));

        Ok(())
    }

    /// Returns a reference to the resource of type `T`.
    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self.resources
            .get(&type_id)
            .and_then(|res| res.downcast_ref::<Res<T>>())
            .map(|res| &res.value)
    }

    /// Returns a mutable reference to the resource of type `T`.
    pub fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self.resources
            .get_mut(&type_id)
            .and_then(|res| res.downcast_mut::<Res<T>>())
            .map(|res| &mut res.value)
    }
}