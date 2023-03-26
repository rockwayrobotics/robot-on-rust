extern crate arfur_robot;

use arfur::prelude::RobotBuilder as ArfurRobotBuilder;
use arfur_robot::robot::Robot;
use arfur_robot::state::manager::Manager;
use tokio::sync::broadcast;

#[tokio::main]
async fn main() -> arfur::wpilib::error::Result<()> {
    let proof_of_initialization = ArfurRobotBuilder::default().initialize()?;
    let robot = Robot::new(proof_of_initialization);
    let (shutdown_send, shutdown_recv) = broadcast::channel(1);
    let mut manager = Manager::new(
        proof_of_initialization,
        &robot,
        robot.ds.get_state_receiver(),
        shutdown_recv,
    );
    manager.manage().await.expect("State manager died");
    shutdown_send.send(()).unwrap();
    Ok(())
}
