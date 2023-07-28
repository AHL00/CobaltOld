use std::{any::Any, collections::HashMap};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct Res<T> {
    id: u32,
    _marker: std::marker::PhantomData<T>,
}

pub struct ResourceManager {
    hashmap: HashMap<u32, Box<dyn Any>>,
    _current_id: u32,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        ResourceManager {
            hashmap: HashMap::new(),
            _current_id: 0,
        }
    }

    /// Allocates a new resource and returns a handle to it. Not as efficient as
    /// `create_from_boxed` but more ergonomic.
    pub fn create<T: 'static>(&mut self, item: T) -> Res<T> {
        let id = self._current_id;
        self._current_id += 1;

        self.hashmap.insert(id, Box::new(item));

        Res {
            id,
            _marker: std::marker::PhantomData,
        }
    }

    /// Adds a resource to the manager.
    pub fn create_from_box<T: 'static>(&mut self, item: Box<T>) -> Res<T> {
        let id = self._current_id;
        self._current_id += 1;

        self.hashmap.insert(id, item);

        Res {
            id,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn get<T: 'static>(&self, res: &Res<T>) -> Option<&T> {
        let item = self.hashmap.get(&res.id);
        if item.is_none() {
            return None;
        }

        item.unwrap().downcast_ref::<T>()
    }

    pub fn get_mut<T: 'static>(&mut self, res: &Res<T>) -> Option<&mut T> {
        let item = self.hashmap.get_mut(&res.id);
        if item.is_none() {
            return None;
        }

        item.unwrap().downcast_mut::<T>()
    }

    pub fn remove<T: 'static>(&mut self, res: &Res<T>) -> Result<(), &str> {
        let item = self.hashmap.remove(&res.id);
        if item.is_none() {
            return Err(ERR_RESOURCE_NOT_FOUND);
        }

        Ok(())
    }
}

const ERR_RESOURCE_NOT_FOUND: &str = "Resource not found";
