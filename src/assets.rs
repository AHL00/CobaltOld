use std::{any::Any, ops::{Deref, DerefMut}, sync::mpsc::{Sender, Receiver}};

use ahash::AHashMap;

/// Lightweight handle to an asset managed by the asset manager
/// Reference counted
pub struct Asset<T> {
    // If this is true it will not increment or decrement the reference count
    id: usize,
    ref_count_sender: Sender<RefCountMessage>,
    data: *mut T,
}

unsafe impl<T> Send for Asset<T> {}
unsafe impl<T> Sync for Asset<T> {}

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
        self.ref_count_sender.send(RefCountMessage::Clone(self.id)).unwrap();

        Asset {
            id: self.id,
            data: self.data,
            ref_count_sender: self.ref_count_sender.clone(),
        }
    }
}

impl<T> Drop for Asset<T> {
    fn drop(&mut self) {
        self.ref_count_sender.send(RefCountMessage::Drop(self.id)).unwrap();
    }
}

enum RefCountMessage {
    Drop(usize),
    Clone(usize),
}

/// Asset manager that implements reference counting
/// Do not destroy this manager while asset handles are still alive
pub struct AssetManager {
    assets: AHashMap<usize, (Box<dyn Any>, usize)>,
    current_id: usize,
    ref_count_sender: Sender<RefCountMessage>,
    ref_count_receiver: Receiver<RefCountMessage>,
}

impl AssetManager {
    pub(crate) fn new() -> AssetManager {
        let (tx, rx) = std::sync::mpsc::channel();
        
        AssetManager {
            assets: AHashMap::new(),
            current_id: 0,
            ref_count_sender: tx,
            ref_count_receiver: rx,
        }
    }

    pub fn create_asset<T: 'static>(&mut self, value: T) -> anyhow::Result<Asset<T>> {
        let id = self.current_id;
        self.current_id += 1;

        self.assets.insert(id, (Box::new(value), 1));

        Ok(Asset {
            id,
            ref_count_sender: self.ref_count_sender.clone(),
            data: self
                .assets
                .get_mut(&id)
                .unwrap()
                .0
                .downcast_mut::<T>()
                .unwrap(),
        })
    }

    pub fn update_ref_counts(&mut self) {
        while let Ok(msg) = self.ref_count_receiver.try_recv() {
            match msg {
                RefCountMessage::Drop(id) => {
                    if let Some(asset) = self.assets.get_mut(&id) {
                        asset.1 -= 1;
                        if asset.1 == 0 {
                            self.assets.remove(&id);
                        }
                    }
                }
                RefCountMessage::Clone(id) => {
                    if let Some(asset) = self.assets.get_mut(&id) {
                        asset.1 += 1;
                    }
                }
            }
        }
    }
}
