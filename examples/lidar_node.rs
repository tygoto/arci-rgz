///
/// This is a Rust rewrite of lidar_node.c from the following tutorial.
/// https://gazebosim.org/docs/harmonic/sensors
///

use anyhow::Result;
use arci::{BaseVelocity, LaserScan2D, MoveBase};
use tokio::signal;
use tokio::time::sleep;
use arci_rgz::{GzMoveBase, Node, GzLaserScan2D};

#[tokio::main]
async fn main() -> Result<()> {
    let node = Node::new(None);
    let node_clone = node.clone();

    let laser_scan = GzLaserScan2D::new(node_clone, "/lidar");
    let move_base = GzMoveBase::new(node, "/cmd_vel");

    tokio::spawn(async move {
        loop {
            if let Ok(msg) = laser_scan.current_scan() {
                let all_more = msg.ranges.iter().all(|&range| range >= 1.0);
                let mut vel = BaseVelocity::default();
                if all_more {
                    vel.x = 0.5;
                    vel.theta = 0.0;
                } else {
                    vel.x = 0.0;
                    vel.theta = 0.5;
                };
                move_base.send_velocity(&vel).unwrap();
            }
            sleep(std::time::Duration::from_millis(100)).await;
        }
    });

    println!("Press Ctrl-C to exit.");
    signal::ctrl_c().await?;

    Ok(())
}
