use crate::{
    state::{ds::DriverStation, manager::ModeBehavior},
    subsystems::{
        drivebase::{DifferentialDrivebase, MotorSpeed},
        Subsystems,
    },
};

use arfur::wpilib::{
    error::Error as ArfurError,
    robot::{Robot as ArfurRobot, RobotBuilder as ArfurRobotBuilder},
};
use async_trait::async_trait;

pub struct Robot {
    pub subsystems: Subsystems,
    pub ds: DriverStation<2>,
}

impl Robot {
    pub fn new(proof_of_initialization: ArfurRobot) -> Self {
        std::thread::spawn(arfur::wpilib::util::create_observer());
        Self {
            subsystems: Subsystems::new(proof_of_initialization),
            ds: DriverStation::new(proof_of_initialization),
        }
    }

    pub fn default_initialize() -> Result<Self, ArfurError> {
        Ok(Self::new(ArfurRobotBuilder::default().initialize()?))
    }
}

#[async_trait]
impl ModeBehavior for Robot {
    async fn disabled(&self) {}
    async fn autonomous(&self) {}

    async fn teleop(&self) {
        let mut drivebase = self.subsystems.drivebase.acquire().await;
        let mut controller = self.ds.get_controller_receiver::<0>();
        loop {
            let Ok(_) = controller.changed().await else { break; };
            let Some(input) = &*controller.borrow_and_update() else { continue; };
            let analog_x = input.axis(0).unwrap_or_default();
            let analog_y = input.axis(1).unwrap_or_default();
            let drive_slow = input.button(0).unwrap_or_default();
            let scale_factor: f64 = if drive_slow { 0.5 } else { 1.0 };
            drivebase.arcade_drive(
                MotorSpeed::new_clamped(f64::from(analog_x) * scale_factor),
                MotorSpeed::new_clamped(f64::from(analog_y) * scale_factor),
            );
        }
    }
}
