extern crate wpilib;

mod wpilibfill;
use wpilibfill::*;

use wpilib::ds::DriverStation;

#[derive(Debug)]
struct Robot {}

impl robot::IterativeRobot for Robot {
    fn new(_ds: &DriverStation) -> Robot {
        Robot {}
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

    fn robot_periodic(&mut self) {}
}

fn main() {
    robot::start_timed::<Robot>()
}
