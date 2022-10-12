// Copyright 2019 First Rust Competition Developers.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// use std::time::Duration;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use wpilib_sys::*;

#[derive(Debug)]
/// An FPGA notifier alarm.
pub struct Alarm {
    handle: HAL_NotifierHandle,
}

impl Alarm {
    pub fn new() -> HalResult<Self> {
        Ok(Alarm {
            handle: hal_call!(HAL_InitializeNotifier())?,
        })
    }

    pub fn stop(&self) -> HalResult<()> {
        hal_call!(HAL_StopNotifier(self.handle))
    }

    /// Updates the trigger time.
    ///
    /// Note that this time is an absolute FPGA timestamp.
    pub fn update(&self, trigger_time: u64) -> HalResult<()> {
        hal_call!(HAL_UpdateNotifierAlarm(self.handle, trigger_time))
    }

    pub fn cancel(&self) -> HalResult<()> {
        hal_call!(HAL_CancelNotifierAlarm(self.handle))
    }

    /// Waits for the next alarm.
    ///
    /// This is a blocking call until either the time elapses or
    /// the stop method is called.
    ///
    /// Returns the FPGA timestamp at which the alarm returned.
    pub fn wait(&self) -> HalResult<u64> {
        hal_call!(HAL_WaitForNotifierAlarm(self.handle))
    }
}

impl Drop for Alarm {
    fn drop(&mut self) {
        let _ = self.stop();
        let _ = hal_call!(HAL_CleanNotifier(self.handle));
    }
}

pub struct Notifier {
    alarm: Arc<Alarm>,
    handle: Option<JoinHandle<()>>,
}

impl Notifier {
    /// Creates a new thread with timing backed by a notifier alarm
    ///
    /// The provided handler will be called with the given period until the notifier is dropped,
    /// at which point the executor thread will be joined. Any long-running code in the handler
    /// will block the thread that `Drop`s the `Notifier`.
    pub fn new(mut handler: impl FnMut() + Send + 'static, period: Duration) -> HalResult<Self> {
        let alarm = Arc::new(Alarm::new()?);

        let thread_alarm = alarm.clone();
        let handle = thread::spawn(move || {
            while let Ok(cur_time) = thread_alarm.wait() {
                if cur_time == 0 {
                    break;
                }

                handler();
                let _ = thread_alarm.update(cur_time + period.as_micros() as u64);
            }
        });

        Ok(Notifier {
            alarm,
            handle: Some(handle),
        })
    }
}

impl Drop for Notifier {
    fn drop(&mut self) {
        let _ = self.alarm.stop();

        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}