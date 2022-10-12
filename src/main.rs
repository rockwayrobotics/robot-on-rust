extern crate wpilib;

mod wpilibfill;
mod constants;
mod drivebase;

use wpilibfill::*;
use wpilib::ds::DriverStation;
use drivebase::Drivebase;

#[derive(Debug)]
struct Robot {
    drivebase: Drivebase,
}

impl robot::IterativeRobot for Robot {
    fn new(_ds: &DriverStation) -> Robot {
        Robot {
            drivebase: Drivebase::new(),
        }
    }

    fn disabled_init(&mut self) {
        println!("Transitioning to disabled.");
    }

    fn autonomous_init(&mut self) {
        println!("Transitioning to autonomous mode.");
    }

    fn teleop_init(&mut self) {
        println!("Transitioning to teleoperated mode.");
    }

    fn test_init(&mut self) {
        println!("Transitioning to test mode.");
    }

    fn disabled_periodic(&mut self) {}

    fn autonomous_periodic(&mut self) {}

    fn teleop_periodic(&mut self) {}

    fn test_periodic(&mut self) {}

    fn robot_periodic(&mut self) {
        self.drivebase.feed_motors();
    }
}

fn main() {
    robot::start_timed::<Robot>()
}
