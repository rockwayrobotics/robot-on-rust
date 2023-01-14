use arfur::rev::prelude::*;
use arfur::wpilib::robot::Robot as ArfurRobot;

#[derive(Debug)]
pub struct Drivebase {
    left_front: SparkMax,
    left_back: SparkMax,
    right_front: SparkMax,
    right_back: SparkMax,
}

impl Drivebase {
    pub fn new(base: ArfurRobot, left_front_can: i32, left_back_can: i32, right_front_can: i32, right_back_can: i32) -> Self {
        Self {
            left_front: SparkMax::new(base, left_front_can),
            left_back: SparkMax::new(base, left_back_can),
            right_front: SparkMax::new(base, right_front_can),
            right_back: SparkMax::new(base, right_back_can),
        }
    }
}

pub trait DifferentialDrivebase {
    unsafe fn set_left_unchecked(&mut self, left: f64);
    unsafe fn set_right_unchecked(&mut self, right: f64);

    fn set_left(&mut self, left: f64) {
        unsafe {
            self.set_left_unchecked(left.clamp(-1.0, 1.0));
        }
    }

    fn set_right(&mut self, right: f64) {
        unsafe {
            self.set_right_unchecked(right.clamp(-1.0, 1.0));
        }
    }

    fn tank_drive(&mut self, left: f64, right: f64) {
        self.set_left(left.clamp(-1.0, 1.0));
        self.set_right(right.clamp(-1.0, 1.0));
    }

    fn arcade_drive(&mut self, y: f64, x: f64) {
        let y = y.clamp(-1.0, 1.0);
        let x = x.clamp(-1.0, 1.0);
        let max_input = y.abs().max(x.abs());
        let min_input = y.abs().min(x.abs());

        let (left, right) = if max_input == 0.0 {
            (0.0, 0.0)
        } else {
            let saturated_input = (max_input + min_input) / max_input;
            let left = (y - x) / saturated_input;
            let right = (y + x) / saturated_input;
            (left, right)
        };

        self.set_left(left);
        self.set_right(right);
    }
}

impl DifferentialDrivebase for Drivebase {
    unsafe fn set_left_unchecked(&mut self, left: f64) {
        self.left_front.set_percentage(left);
        self.left_back.set_percentage(left);
    }

    unsafe fn set_right_unchecked(&mut self, right: f64) {
        self.right_front.set_percentage(right);
        self.right_back.set_percentage(right);
    }
}
