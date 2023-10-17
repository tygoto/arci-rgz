use anyhow::Result;
use arci::{BaseVelocity, MoveBase};
use arci_rgz::{GzCmdVelMoveBase, GzClient};

#[tokio::main]
async fn main() -> Result<()> {

    let client = GzClient::new(None);
    let c = GzCmdVelMoveBase::new(&client, "/cmd_vel");

    let mut count = 0;
    let mut vel = BaseVelocity::default();
    while count < 100 {
        vel.x = 0.01 * (count as f64);
        c.send_velocity(&vel)?;
        count += 1;
        println!("{count}, {vel:?}");
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    while count >= 0 {
        vel.x = -0.01 * (count as f64);
        c.send_velocity(&vel)?;
        count -= 1;
        println!("{count}, {vel:?}");
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    Ok(())
}