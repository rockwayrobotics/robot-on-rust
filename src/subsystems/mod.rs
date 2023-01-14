pub mod drivebase;

use arfur::wpilib::robot::Robot as ArfurRobot;

use self::drivebase::Drivebase;

#[derive(Debug)]
pub struct Subsystems {
    pub drivebase: Drivebase,
}

impl Subsystems {
    pub fn default_from_base(base: ArfurRobot) -> Self {
        Self {
            drivebase: Drivebase::new(base, 0, 1, 2, 3),
        }
    }
}
