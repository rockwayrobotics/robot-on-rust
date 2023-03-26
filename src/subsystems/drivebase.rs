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
    pub fn new(
        proof_of_initialization: ArfurRobot,
        left_front_can: i32,
        left_back_can: i32,
        right_front_can: i32,
        right_back_can: i32,
    ) -> Self {
        Self {
            left_front: SparkMax::new(proof_of_initialization, left_front_can),
            left_back: SparkMax::new(proof_of_initialization, left_back_can),
            right_front: SparkMax::new(proof_of_initialization, right_front_can),
            right_back: SparkMax::new(proof_of_initialization, right_back_can),
        }
    }
}

// TODO: this should probably be on SparkMax instead
unsafe impl Send for Drivebase {}

pub struct MotorSpeed(f64);

impl MotorSpeed {
    /// Creates a motor speed without checking if the input is between -1 and 1 inclusive.
    /// This may lead to motor burn-out if the value is outside this range.
    /// # Safety
    /// The value must be within the range `[-1.0, 1.0]`.
    pub unsafe fn new_unchecked(speed: f64) -> Self {
        Self(speed)
    }

    /// Creates a motor speed if the given value is between -1 and 1 inclusive.
    pub fn new(speed: f64) -> Option<Self> {
        if (-1.0..=1.0).contains(&speed) {
            Some(Self(speed))
        } else {
            None
        }
    }

    /// Creates a motor speed, clamping the input to `[-1.0, 1.0]`.
    /// ```rust
    /// let too_high = MotorSpeed::new_clamped(2.0);
    /// assert_eq!(too_high.get(), 1.0);
    /// let too_low = MotorSpeed::new_clamped(-2.0);
    /// assert_eq!(too_low.get(), -1.0);
    /// let in_range = MotorSpeed::new_clamped(-0.5);
    /// assert_eq!(in_range.get(), -0.5);
    /// ```
    pub fn new_clamped(speed: f64) -> Self {
        Self(speed.clamp(-1.0, 1.0))
    }

    /// Returns the value as a primitive type.
    pub fn get(&self) -> f64 {
        self.0
    }
}

pub trait DifferentialDrivebase {
    fn set_left(&mut self, left: MotorSpeed);
    fn set_right(&mut self, right: MotorSpeed);

    fn tank_drive(&mut self, left: MotorSpeed, right: MotorSpeed) {
        self.set_left(left);
        self.set_right(right);
    }

    fn arcade_drive(&mut self, y: MotorSpeed, x: MotorSpeed) {
        let y = y.get();
        let x = x.get();
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

        self.set_left(MotorSpeed::new_clamped(left));
        self.set_right(MotorSpeed::new_clamped(right));
    }
}

impl DifferentialDrivebase for Drivebase {
    fn set_left(&mut self, left: MotorSpeed) {
        self.left_front.set_percentage(left.get());
        self.left_back.set_percentage(left.get());
    }

    fn set_right(&mut self, right: MotorSpeed) {
        self.right_front.set_percentage(right.get());
        self.right_back.set_percentage(right.get());
    }
}
