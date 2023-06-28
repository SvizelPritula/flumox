use std::{collections::HashMap, hash::Hash, mem::ManuallyDrop, sync::Arc};

use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use tokio::sync::broadcast::{self, error::RecvError};

struct Inner<K, M> {
    map: HashMap<K, broadcast::Sender<M>>,
    capacity: usize,
}

type SharedInner<K, M> = Arc<RwLock<Inner<K, M>>>;

pub struct ChannelMap<K, M> {
    inner: SharedInner<K, M>,
}

pub struct Receiver<K: Hash + Eq, M> {
    inner: ManuallyDrop<broadcast::Receiver<M>>,
    map: SharedInner<K, M>,
    key: K,
}

impl<K: Clone + Hash + Eq, M: Clone> ChannelMap<K, M> {
    pub fn new(capacity: usize) -> Self {
        ChannelMap {
            inner: Arc::new(RwLock::new(Inner {
                map: HashMap::new(),
                capacity,
            })),
        }
    }

    pub fn send(&self, key: &K, value: M) {
        let inner = self.inner.read();

        if let Some(sender) = inner.map.get(key) {
            let _ = sender.send(value);
        }
    }

    pub fn subscribe(&self, key: K) -> Receiver<K, M> {
        let inner = self.inner.upgradable_read();

        let receiver = if let Some(sender) = inner.map.get(&key) {
            sender.subscribe()
        } else {
            let (sender, receiver) = broadcast::channel(inner.capacity);
            let mut inner = RwLockUpgradableReadGuard::upgrade(inner);

            inner.map.insert(key.clone(), sender);
            receiver
        };

        Receiver {
            inner: ManuallyDrop::new(receiver),
            map: self.inner.clone(),
            key,
        }
    }
}

impl<K: Hash + Eq, M: Clone> Receiver<K, M> {
    pub async fn recv(&mut self) -> Result<M, RecvError> {
        self.inner.recv().await
    }
}

impl<K: Eq + Hash, M> Drop for Receiver<K, M> {
    fn drop(&mut self) {
        let mut inner = self.map.write();

        unsafe {
            // Safety: self.inner is never used again since this struct is dropped
            ManuallyDrop::drop(&mut self.inner);
        }

        if let Some(sender) = inner.map.get(&self.key) {
            if sender.receiver_count() <= 1 {
                inner.map.remove(&self.key);
            }
        }
    }
}

impl<K: Clone + Hash + Eq, M: Clone> Default for ChannelMap<K, M> {
    fn default() -> Self {
        ChannelMap::new(16)
    }
}

impl<K, M> Clone for ChannelMap<K, M> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
