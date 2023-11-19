use std::sync::Arc;
use arci::*;
use serde::{Deserialize, Serialize};
use parking_lot::Mutex;

use rgz::msgs::LaserScan;
use tokio::sync::mpsc;

use crate::{Node};

/// `arci::LaserScan2D` implementation for Gazebo.
pub struct GzLaserScan2D {
    node: Node,
    laser_scan_topic_name: String,
    receiver: Arc<Mutex<mpsc::Receiver<LaserScan>>>,
}

impl GzLaserScan2D {
    pub fn new(node: Node, laser_scan_topic_name: &str) -> Self {
        let receiver =  node.subscribe::<LaserScan>(laser_scan_topic_name).unwrap();
        Self {
            node,
            laser_scan_topic_name: laser_scan_topic_name.to_owned(),
            receiver: Arc::new(Mutex::new(receiver)),
        }
    }
}

impl LaserScan2D for GzLaserScan2D {
    fn current_scan(&self) -> Result<Scan2D, Error> {
        let current_scan = match self.receiver.lock().try_recv() {
            Ok(msg) => Scan2D {
                angle_min: msg.angle_min,
                angle_max: msg.angle_max,
                angle_increment: msg.angle_step,
                time_increment: 0.0,  // Not supported in gz::msgs::LaserScan.
                scan_time: 0.0,  // Not supported in gz::msgs::LaserScan.
                range_min: msg.range_min,
                range_max: msg.range_max,
                ranges: msg.ranges.iter().map(|&v| v).collect::<Vec<f64>>(),
                intensities: msg
                    .intensities
                    .iter()
                    .map(|&v| v)
                    .collect::<Vec<f64>>(),
            },
            Err(_) => {
                return Err(Error::Connection {
                    message: format!("Failed to get scan from {}", self.laser_scan_topic_name),
                })
            }
        };

        Ok(current_scan)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GzLaserScan2DConfig {
    pub topic: String,
}
