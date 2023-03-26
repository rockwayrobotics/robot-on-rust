use std::{future::Future, pin::Pin};

use arfur::prelude::Robot as ArfurRobot;

use super::ds::RobotMode;
use async_trait::async_trait;
use tokio::sync::watch::error::RecvError;
use tokio::sync::{broadcast, watch};

#[async_trait]
pub trait ModeBehavior {
    async fn autonomous(&self);
    async fn teleop(&self);
    async fn disabled(&self);
}

pub struct Manager<'a, T: ModeBehavior> {
    behaviors: &'a T,
    incoming_states: watch::Receiver<RobotMode>,
    shutdown: broadcast::Receiver<()>,
}

impl<'a, T: ModeBehavior> Manager<'a, T> {
    pub fn new(
        _proof_of_inititalization: ArfurRobot,
        robot: &'a T,
        incoming_states: watch::Receiver<RobotMode>,
        shutdown: broadcast::Receiver<()>,
    ) -> Self {
        Self {
            behaviors: robot,
            incoming_states,
            shutdown,
        }
    }

    pub async fn manage(&mut self) -> Result<(), RecvError> {
        loop {
            let future: Pin<Box<dyn Future<Output = ()>>> =
                match *self.incoming_states.borrow_and_update() {
                    RobotMode::Autonomous => self.behaviors.autonomous(),
                    RobotMode::Teleop => self.behaviors.teleop(),
                    RobotMode::Disabled => self.behaviors.disabled(),
                    RobotMode::EStop => Box::pin(async {}),
                };
            tokio::select! {
                _ = self.shutdown.recv() => {return Ok(());}
                val = self.incoming_states.changed() => {val?;}
                _ = future => {self.incoming_states.changed().await?;}
            };
        }
    }
}
