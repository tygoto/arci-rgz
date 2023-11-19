use std::sync::Arc;
use anyhow::{Result};

use parking_lot::{Mutex, MutexGuard};
use tracing::{error};
use tokio;
use tokio::sync::mpsc::{self, Receiver};

use rgz::transport;
use rgz::msgs::{GzMessage};


pub struct NodeConfig {

}

#[derive(Clone)]
pub struct Node {
    inner: Arc<Mutex<transport::Node>>,

}

impl Node {
    pub fn new(config: Option<NodeConfig>) -> Self {
        let gz_node = transport::Node::new(None);

        Self {
            inner: Arc::new(Mutex::new(gz_node)),
        }
    }
    pub fn create_publisher<T>(&self, topic: &str) -> Result<transport::Publisher<T>>
        where T: GzMessage
    {
        let publisher = {
            let node = self.inner.lock();
            node.advertise::<T>(topic, None)?
        };
        Ok(publisher)
    }

    pub fn subscribe<T>(&self, topic: &str) -> Result<Receiver<T>>
        where
            T: GzMessage + Default + 'static,
    {
        let (sender, mut receiver) = mpsc::channel::<T>(10);
        {
            let mut node = self.inner.lock();
            node.subscribe(topic, move |msg: T| {
                if let Err(err) = sender.try_send(msg) {
                    error!("Failed to send message: {}", err);
                }
            })?;
        };
        Ok(receiver)
    }
}

