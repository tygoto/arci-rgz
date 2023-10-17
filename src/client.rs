use std::{borrow::Cow, collections::HashMap, mem, sync::Arc, thread::sleep, time::Duration};

use anyhow::{format_err, Result};

use arci::{
    nalgebra as na, BaseVelocity, JointPositionLimit, JointPositionLimiter, JointTrajectoryClient,
    JointVelocityLimiter, Localization, MoveBase, Navigation, TrajectoryPoint, WaitFuture,
};
use parking_lot::Mutex;
use rgz::msgs::{GzMessage, Twist};
use schemars::JsonSchema;
use scoped_sleep::ScopedSleep;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;
use tracing::debug;
// use crate::utils::*;

use rgz::transport;
use rgz::transport::Publisher;

pub struct ClientConfig {

}

pub struct GzClient {
    node: transport::Node,
}
impl GzClient {
    pub fn new(config: Option<ClientConfig>) -> Self {
        let node = transport::Node::new(None);

        Self {
            node,
        }
    }

    pub fn node(&self) -> &transport::Node {
        &self.node
    }

}

