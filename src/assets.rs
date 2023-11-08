use std::{any::Any, ops::{Deref, DerefMut}};

use ahash::AHashMap;

/// Lightweight handle to an asset managed by the asset manager
/// Reference counted
pub struct Asset<T> {
    // If this is true it will not increment or decrement the reference count
    id: usize,
    asset_manager_ptr: *mut AssetManager,
    data: *mut T,
}

impl<T> Asset<T> {
    // fn clone_weak(&self) -> AssetWeak<T>
}

impl<T> AsRef<T> for Asset<T> {
    fn as_ref(&self) -> &T {
        unsafe { &*self.data }
    }
}

impl<T> AsMut<T> for Asset<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.data }
    }
}

impl<T> Deref for Asset<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.data }
    }
}

impl<T> DerefMut for Asset<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.data }
    }
}

impl<T> Clone for Asset<T> {
    fn clone(&self) -> Self {
        unsafe {
            let asset_manager = &mut *self.asset_manager_ptr;
            let asset = asset_manager.assets.get_mut(&self.id).unwrap();
            asset.1 += 1;
        }
        Asset {
            id: self.id,
            asset_manager_ptr: self.asset_manager_ptr,
            data: self.data,
        }
    }
}

impl<T> Drop for Asset<T> {
    fn drop(&mut self) {
        unsafe {
            let asset_manager = &mut *self.asset_manager_ptr;
            let asset = asset_manager.assets.get_mut(&self.id).unwrap();
            asset.1 -= 1;
            if asset.1 == 0 {
                asset_manager.assets.remove(&self.id);
            }
        }
    }
}

/// Asset manager that implements reference counting
/// Do not destroy this manager while asset handles are still alive
pub struct AssetManager {
    assets: AHashMap<usize, (Box<dyn Any>, usize)>,
    current_id: usize,
}

impl AssetManager {
    pub(crate) fn new() -> AssetManager {
        AssetManager {
            assets: AHashMap::new(),
            current_id: 0,
        }
    }

    pub fn create_asset<T: 'static>(&mut self, value: T) -> anyhow::Result<Asset<T>> {
        let id = self.current_id;
        self.current_id += 1;

        self.assets.insert(id, (Box::new(value), 1));

        Ok(Asset {
            id,
            asset_manager_ptr: self as *mut AssetManager,
            data: self
                .assets
                .get_mut(&id)
                .unwrap()
                .0
                .downcast_mut::<T>()
                .unwrap(),
        })
    }
}
