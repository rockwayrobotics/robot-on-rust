extern crate arfur_robot;

use arfur_robot::joystick::*;
use arfur_robot::robot::Robot;
use arfur_robot::subsystems::drivebase::DifferentialDrivebase;

fn main() -> arfur::wpilib::error::Result<()> {
    let mut robot = Robot::default_initialize()?;
    let mut joystick = Joystick::new(JoystickPort::new(0).unwrap());
    loop {
        let analog_x = joystick.get_stick_axis(JoystickAxis::Axis0).unwrap();
        let analog_y = joystick.get_stick_axis(JoystickAxis::Axis1).unwrap();
        let drive_slow = joystick.get_button_pressed(JoystickButton::new(0).unwrap()).unwrap();
        let scale_factor: f64 = if drive_slow { 0.5 } else { 1.0 };
        robot.subsystems.drivebase.arcade_drive(
            analog_x as f64 * scale_factor,
            analog_y as f64 * scale_factor,
        );
    }
}
