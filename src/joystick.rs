use arfur::wpilib::ffi::root as wpi;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum JoystickError {
    PortDoesNotExist,
    ButtonUnplugged,
    AxisUnplugged,
    AxisDoesNotExist,
    PovDoesNotExist,
    PovUnplugged,
}

#[derive(Copy, Clone, Debug)]
pub struct JoystickPort(pub i32);

impl JoystickPort {
    pub fn new(port: u8) -> Result<JoystickPort, JoystickError> {
        if port as u32 >= wpi::HAL_kMaxJoysticks {
            return Err(JoystickError::PortDoesNotExist);
        }
        Ok(JoystickPort(i32::from(port)))
    }
}

#[derive(Copy, Clone, Debug)]
pub struct JoystickButton(pub i32);

impl JoystickButton {
    pub fn new(button: u8) -> Result<JoystickButton, JoystickError> {
        Ok(JoystickButton(i32::from(button)))
    }
}

#[derive(Copy, Clone, Debug)]
pub enum JoystickPOV {
    Pov0 = 0,
    Pov1 = 1,
    Pov2 = 2,
    Pov3 = 3,
    Pov4 = 4,
    Pov5 = 5,
    Pov6 = 6,
    Pov7 = 7,
    Pov8 = 8,
    Pov9 = 9,
    Pov10 = 10,
    Pov11 = 11,
    Pov12 = 12,
}

#[derive(Copy, Clone, Debug)]
pub enum JoystickAxis {
    Axis0 = 0,
    Axis1 = 1,
    Axis2 = 2,
    Axis3 = 3,
    Axis4 = 4,
    Axis5 = 5,
    Axis6 = 6,
    Axis7 = 7,
    Axis8 = 8,
    Axis9 = 9,
    Axis10 = 10,
    Axis11 = 11,
    Axis12 = 12,
}

pub struct Joystick {
    port: JoystickPort,
    buttons: wpi::HAL_JoystickButtons,
    axes: wpi::HAL_JoystickAxes,
    povs: wpi::HAL_JoystickPOVs,
}

impl Joystick {
    pub fn new(port: JoystickPort) -> Joystick {
        Self {
            port,
            buttons: wpi::HAL_JoystickButtons { buttons: 0, count: 0 },
            axes: wpi::HAL_JoystickAxes { axes: [0_f32; 12], count: 0 },
            povs: wpi::HAL_JoystickPOVs { povs: [0_i16; 12], count: 0},
        }
    }

    pub fn get_button_pressed(&mut self, button: JoystickButton) -> Result<bool, JoystickError> {
        unsafe {
            wpi::HAL_GetJoystickButtons(self.port.0, &mut self.buttons);
        }

        if button.0 >= self.buttons.count as i32 {
            return Err(JoystickError::ButtonUnplugged);
        }

        Ok(self.buttons.buttons & (1 << button.0) != 0)
    }

    pub fn get_stick_axis(&mut self, axis: JoystickAxis) -> Result<f32, JoystickError> {
        unsafe {
            wpi::HAL_GetJoystickAxes(self.port.0, &mut self.axes);
        }

        if axis as i16 > self.axes.count {
            return Err(JoystickError::AxisUnplugged);
        }

        Ok(self.axes.axes[axis as usize])
    }

    pub fn get_stick_pov(&mut self, pov: JoystickPOV) -> Result<i16, JoystickError> {
        unsafe {
            wpi::HAL_GetJoystickPOVs(self.port.0, &mut self.povs);
        }

        if pov as i16 > self.povs.count {
            return Err(JoystickError::PovUnplugged);
        }

        Ok(self.povs.povs[pov as usize])
    }
}
