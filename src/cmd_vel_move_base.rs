use arci::*;
use crate::client::GzClient;

use rgz::msgs::{Twist, Vector3d};
use rgz::transport::Publisher;
use tracing::info;

pub struct GzCmdVelMoveBase {
    publisher: Publisher<Twist>,
}

impl GzCmdVelMoveBase {
    pub fn new(client: &GzClient, cmd_topic_name: &str) -> Self {
        let publisher = client.node().advertise::<Twist>(cmd_topic_name, None).unwrap();
        Self {
            publisher,
        }
    }
}

impl MoveBase for GzCmdVelMoveBase {
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

