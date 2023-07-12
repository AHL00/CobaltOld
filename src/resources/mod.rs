use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub struct Res<T> {
    id: u32,
    _marker: std::marker::PhantomData<T>,
}

pub struct ResourceManager {
    hashmaps: HashMap<TypeId, HashMap<u32, Box<dyn Any>>>,
    _current_id: u32,
}

impl ResourceManager {
    pub fn new() -> ResourceManager {
        ResourceManager {
            hashmaps: HashMap::new(),
            _current_id: 0,
        }
    }

    pub fn add<T: 'static>(&mut self, item: T) -> Res<T>
    {
        let type_id = TypeId::of::<T>();
        let hashmap = self.hashmaps.entry(type_id).or_insert(HashMap::new());
        let id = self._current_id;
        self._current_id += 1;
        hashmap.insert(id, Box::new(item));
        Res {
            id,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn get<T: 'static>(&self, res: &Res<T>) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        let hashmap = self.hashmaps.get(&type_id)?;
        let item = hashmap.get(&res.id)?;
        item.downcast_ref::<T>()
    }

    pub fn get_mut<T: 'static>(&mut self, res: &Res<T>) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        let hashmap = self.hashmaps.get_mut(&type_id)?;
        let item = hashmap.get_mut(&res.id)?;
        item.downcast_mut::<T>()
    }

    pub fn remove<T: 'static>(&mut self, id: u32) -> Option<Box<T>> {
        let type_id = TypeId::of::<T>();
        let hashmap = self.hashmaps.get_mut(&type_id)?;
        let item = hashmap.remove(&id)?;
        item.downcast::<T>().ok()
    }
}
