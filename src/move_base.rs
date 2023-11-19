use arci::*;
use rgz::msgs::{Twist, Vector3d};
use rgz::transport;

use crate::Node;
use tracing::info;

pub struct GzMoveBase {
    publisher: transport::Publisher<Twist>,
}

impl GzMoveBase {
    pub fn new(node: Node, cmd_topic_name: &str) -> Self {
        let publisher = node.create_publisher(cmd_topic_name).unwrap();
        Self {
            publisher,
        }
    }
}

impl MoveBase for GzMoveBase {
    fn send_velocity(&self, velocity: &BaseVelocity) -> Result<(), Error> {
        let mut twist = Twist { ..Default::default() };
        twist.linear = Some(Vector3d {
            x: velocity.x,
            y: velocity.y,
            ..Default::default()
        });
        twist.angular = Some(Vector3d { z: velocity.theta, ..Default::default() });

        if !self.publisher.is_ready() {
            return Ok(())
        }

        self.publisher.publish(twist).map_err(|e| Error::Connection {
            message: format!("rgz publish error: {e:?}"),
        })
    }

    fn current_velocity(&self) -> Result<BaseVelocity, Error> {
        unimplemented!("Read from /odom in the future?");
    }
}

