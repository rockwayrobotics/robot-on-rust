#![allow(dead_code)]
extern crate ctre;
use ctre::mot::{VictorSPX, MotorController, ControlMode, Demand};

use crate::constants::can;

#[derive(Debug)]
pub struct Drivebase {
    left1: VictorSPX,
    left2: VictorSPX,
    right1: VictorSPX,
    right2: VictorSPX,
    left_power: f64,
    right_power: f64,
}

impl Drivebase {
    pub fn new() -> Self {
        Self {
            left1: VictorSPX::new(can::DRIVE_LEFT1),
            left2: VictorSPX::new(can::DRIVE_LEFT2),
            right1: VictorSPX::new(can::DRIVE_RIGHT1),
            right2: VictorSPX::new(can::DRIVE_RIGHT2),
            left_power: 0_f64,
            right_power: 0_f64,
        }
    }

    /// Drives the robot by speed and rotation.
    /// 
    /// # Arguments
    /// 
    /// * `x` - forward/reverse speed of the robot, between -1 and 1. Forwards is positive.
    /// * `y` - left/right rotation of the robot, between -1 and 1. Right is positive.
    /// * `square_inputs` - makes the input less sensitive at lower speeds. Recommended if passing in analog stick values.
    pub fn arcade_drive(&mut self, x: f64, y: f64, square_inputs: bool) {
        let mut x = x.clamp(-1.0, 1.0);
        let mut y = y.clamp(-1.0, 1.0);

        if square_inputs {
            x = x.powi(2).copysign(x);
            y = y.powi(2).copysign(y);
        }

        let max_input = x.abs().max(y.abs()).copysign(x);

        let (mut left_speed, mut right_speed) = match (x.is_sign_positive(), y.is_sign_positive()) {
            (true, true) | (false, false) => (max_input, x - y),
            (true, false) | (false, true) => (x + y, max_input),
        };

        let max_magnitude = left_speed.abs().max(right_speed.abs());
        if max_magnitude > 1.0 {
            left_speed /= max_magnitude;
            right_speed /= max_magnitude;
        }

        self.left_power = left_speed;
        self.right_power = right_speed;
    }

    /// Drives the robot by left- and right-side speeds.
    /// 
    /// # Arguments
    /// 
    /// * `left` - left-side speed of the robot, between -1 and 1. Forwards is positive.
    /// * `right` - right-side speed of the robot, between -1 and 1. Forwards is positive.
    /// * `square_inputs` - makes the input less sensitive at lower speeds.
    pub fn tank_drive(&mut self, left: f64, right: f64, square_inputs: bool) {
        let mut left = left.clamp(-1.0, 1.0);
        let mut right = right.clamp(-1.0, 1.0);

        if square_inputs {
            left = left.powi(2).copysign(left);
            right = right.powi(2).copysign(right);
        }

        self.left_power = left;
        self.right_power = right;
    }

    /// Sends the latest speed values to the motor controllers.
    /// Must be called periodically to avoid tripping watchdogs.
    pub fn feed_motors(&mut self) {
        self.left1.set(ControlMode::PercentOutput, self.left_power, Demand::Neutral);
        self.left2.set(ControlMode::PercentOutput, self.left_power, Demand::Neutral);
        self.right1.set(ControlMode::PercentOutput, self.right_power, Demand::Neutral);
        self.right2.set(ControlMode::PercentOutput, self.right_power, Demand::Neutral);
    }
}