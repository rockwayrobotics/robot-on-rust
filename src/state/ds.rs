use std::mem::MaybeUninit;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use arfur::prelude::Robot;
use arfur::wpilib::ffi::root as wpi;
use tokio::sync::watch;

#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub enum RobotMode {
    #[default]
    Disabled,
    Autonomous,
    Teleop,
    EStop,
}

#[derive(Debug, PartialEq)]
pub struct ControllerAxes {
    count: i16,
    axes: [f32; 12],
}

impl ControllerAxes {
    fn get(&self, n: u8) -> Option<f32> {
        if i16::from(n) < self.count {
            Some(self.axes[usize::from(n)])
        } else {
            None
        }
    }
}

impl From<wpi::HAL_JoystickAxes> for ControllerAxes {
    fn from(value: wpi::HAL_JoystickAxes) -> Self {
        Self {
            count: value.count,
            axes: value.axes,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ControllerPOVs {
    count: i16,
    povs: [i16; 12],
}

impl ControllerPOVs {
    fn get(&self, n: u8) -> Option<i16> {
        if i16::from(n) < self.count {
            Some(self.povs[usize::from(n)])
        } else {
            None
        }
    }
}

impl From<wpi::HAL_JoystickPOVs> for ControllerPOVs {
    fn from(value: wpi::HAL_JoystickPOVs) -> Self {
        Self {
            count: value.count,
            povs: value.povs,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ControllerButtons {
    count: u8,
    buttons: u32,
}

impl ControllerButtons {
    fn get(&self, n: u8) -> Option<bool> {
        if n < self.count {
            Some(self.buttons & (1 << n) != 0)
        } else {
            None
        }
    }
}

impl From<wpi::HAL_JoystickButtons> for ControllerButtons {
    fn from(value: wpi::HAL_JoystickButtons) -> Self {
        Self {
            count: value.count,
            buttons: value.buttons,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ControllerState {
    axes: ControllerAxes,
    povs: ControllerPOVs,
    buttons: ControllerButtons,
}

impl ControllerState {
    /// Gets the value of a controller axis, or None if no such axis exists.
    pub fn axis(&self, n: u8) -> Option<f32> {
        self.axes.get(n)
    }

    /// Gets the value of a controller POV (D-pad), or None if no such POV exists.
    pub fn pov(&self, n: u8) -> Option<i16> {
        self.povs.get(n)
    }

    /// Gets the value of a controller button, or None if no such button exists.
    pub fn button(&self, n: u8) -> Option<bool> {
        self.buttons.get(n)
    }
}

fn make_channels<const N: usize, T: Default>() -> ([watch::Sender<T>; N], [watch::Receiver<T>; N]) {
    let channels: [_; N] = std::array::from_fn(|_| watch::channel(T::default()));
    let (senders, receivers): (Vec<_>, Vec<_>) = channels.into_iter().unzip();
    let (Ok(senders), Ok(receivers)) = (senders.try_into(), receivers.try_into()) else { unreachable!() };
    (senders, receivers)
}

pub struct DriverStation<const CONTROLLERS: usize> {
    robot_state: watch::Receiver<RobotMode>,
    controllers: [watch::Receiver<Option<ControllerState>>; CONTROLLERS],
    shutdown: Arc<AtomicBool>,
}

impl<const CONTROLLERS: usize> DriverStation<CONTROLLERS> {
    pub fn new(_robot: Robot) -> Self {
        let (event_send, event_recv) = watch::channel(RobotMode::default());
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown2 = Arc::clone(&shutdown);
        let (controller_sends, controller_recvs) =
            make_channels::<{ CONTROLLERS }, Option<ControllerState>>();
        std::thread::spawn(move || {
            let shutdown = shutdown2;
            'outer: loop {
                if shutdown.load(Ordering::Relaxed) {
                    break;
                }
                unsafe { wpi::HAL_WaitForDSDataTimeout(1.0) };

                let new_state = Self::get_state();
                if new_state != *event_send.borrow() && event_send.send(new_state).is_err() {
                    break;
                }

                for i in 0..=CONTROLLERS {
                    let new_controller_state = Self::get_controller(i32::try_from(i).unwrap());
                    let sender = &controller_sends[i];
                    if new_controller_state == *sender.borrow() {
                        continue;
                    }
                    let Ok(_) = sender.send(new_controller_state) else {
                        break 'outer;
                    };
                }
            }
        });
        Self {
            robot_state: event_recv,
            controllers: controller_recvs,
            shutdown,
        }
    }

    /// Returns a watch channel receiver to read the robot's state (Disabled, Autonomous, etc.).
    /// See the RobotState enum for possible states.
    pub fn get_state_receiver(&self) -> watch::Receiver<RobotMode> {
        self.robot_state.clone()
    }

    /// Returns a watch channel receiver to read the state of a controller on a given port.
    /// The channel will yield None when no controller is plugged into the port.
    /// Will panic if a port is requested that is not being listened on.
    /// TODO: can we make this a compile error instead?
    pub fn get_controller_receiver<const PORT: u8>(
        &self,
    ) -> watch::Receiver<Option<ControllerState>> {
        self.controllers[PORT as usize].clone()
    }

    fn get_state() -> RobotMode {
        // SAFETY: safe because HAL_GetControlWord is guaranteed to initialize control_word
        let control_word = unsafe {
            let mut control_word = MaybeUninit::uninit();
            wpi::HAL_GetControlWord(control_word.as_mut_ptr());
            control_word.assume_init()
        };
        if control_word.enabled() != 0 {
            if control_word.autonomous() != 0 {
                RobotMode::Autonomous
            } else {
                RobotMode::Teleop
            }
        } else if control_word.eStop() != 0 {
            RobotMode::EStop
        } else {
            RobotMode::Disabled
        }
    }

    fn get_controller(n: i32) -> Option<ControllerState> {
        let axes = unsafe {
            let mut axes = MaybeUninit::uninit();
            let result = wpi::HAL_GetJoystickAxes(n, axes.as_mut_ptr());
            if result != 0 {
                return None;
            }
            axes.assume_init()
        };
        let povs = unsafe {
            let mut povs = MaybeUninit::uninit();
            let result = wpi::HAL_GetJoystickPOVs(n, povs.as_mut_ptr());
            if result != 0 {
                return None;
            }
            povs.assume_init()
        };
        let buttons = unsafe {
            let mut buttons = MaybeUninit::uninit();
            let result = wpi::HAL_GetJoystickButtons(n, buttons.as_mut_ptr());
            if result != 0 {
                return None;
            }
            buttons.assume_init()
        };
        Some(ControllerState {
            axes: ControllerAxes::from(axes),
            povs: ControllerPOVs::from(povs),
            buttons: ControllerButtons::from(buttons),
        })
    }
}

impl<const CONTROLLERS: usize> Drop for DriverStation<CONTROLLERS> {
    fn drop(&mut self) {
        self.shutdown.store(true, Ordering::Relaxed);
    }
}
