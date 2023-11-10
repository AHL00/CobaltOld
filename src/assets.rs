use std::{any::Any, ops::{Deref, DerefMut}, sync::mpsc::{Sender, Receiver}};

use ahash::AHashMap;

/// Lightweight handle to an asset managed by the asset manager
/// Reference counted
pub struct Asset<T> {
    // If this is true it will not increment or decrement the reference count
    id: usize,
    drop_sender: Sender<usize>,
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
        Asset {
            id: self.id,
            data: self.data,
            drop_sender: self.drop_sender.clone(),
        }
    }
}

impl<T> Drop for Asset<T> {
    fn drop(&mut self) {
        self.drop_sender.send(self.id).unwrap();
    }
}

/// Asset manager that implements reference counting
/// Do not destroy this manager while asset handles are still alive
pub struct AssetManager {
    assets: AHashMap<usize, (Box<dyn Any>, usize)>,
    current_id: usize,
    drop_sender: Sender<usize>,
    drop_receiver: Receiver<usize>,
}

impl AssetManager {
    pub(crate) fn new() -> AssetManager {
        let (tx, rx) = std::sync::mpsc::channel();
        AssetManager {
            assets: AHashMap::new(),
            current_id: 0,
            drop_sender: tx,
            drop_receiver: rx,
        }
    }

    pub fn create_asset<T: 'static>(&mut self, value: T) -> anyhow::Result<Asset<T>> {
        let id = self.current_id;
        self.current_id += 1;

        self.assets.insert(id, (Box::new(value), 1));

        Ok(Asset {
            id,
            drop_sender: self.drop_sender.clone(),
            data: self
                .assets
                .get_mut(&id)
                .unwrap()
                .0
                .downcast_mut::<T>()
                .unwrap(),
        })
    }

    pub fn drop_unused_assets(&mut self) {
        while let Ok(id) = self.drop_receiver.try_recv() {
            if let Some(asset) = self.assets.get_mut(&id) {
                asset.1 -= 1;
                if asset.1 == 0 {
                    self.assets.remove(&id);
                }
            }
        }
    }
}
