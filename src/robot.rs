use crate::subsystems::Subsystems;

use arfur::wpilib::{error::Error as ArfurError, robot::{Robot as ArfurRobot, RobotBuilder as ArfurRobotBuilder}};

pub struct Robot {
    pub subsystems: Subsystems,
}

impl Robot {
    fn from_base(base: ArfurRobot) -> Self {
        std::thread::spawn(arfur::wpilib::util::create_observer());
        Self {
            subsystems: Subsystems::default_from_base(base),
        }
    }

    pub fn default_initialize() -> Result<Self, ArfurError> {
        Ok(Self::from_base(ArfurRobotBuilder::default().initialize()?))
    }
}
