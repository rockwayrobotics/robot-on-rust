pub mod drivebase;

use arfur::wpilib::robot::Robot as ArfurRobot;
use tokio::sync::{Mutex, MutexGuard, TryLockError};

use self::drivebase::Drivebase;

#[derive(Debug, Default)]
pub struct Subsystem<T> {
    inner: Mutex<T>,
}

impl<T> Subsystem<T> {
    pub async fn acquire(&self) -> MutexGuard<T> {
        self.inner.lock().await
    }

    pub fn try_acquire(&self) -> Result<MutexGuard<T>, TryLockError> {
        self.inner.try_lock()
    }
}

impl<T> From<T> for Subsystem<T> {
    fn from(value: T) -> Self {
        Self {
            inner: Mutex::new(value),
        }
    }
}

#[derive(Debug)]
pub struct Subsystems {
    pub drivebase: Subsystem<Drivebase>,
}

impl Subsystems {
    pub fn new(proof_of_initialization: ArfurRobot) -> Self {
        Self {
            drivebase: Subsystem::from(Drivebase::new(proof_of_initialization, 0, 1, 2, 3)),
        }
    }
}
