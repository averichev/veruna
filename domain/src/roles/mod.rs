use std::sync::Arc;
use std::thread;
use serde::{Deserialize, Serialize};
use crate::users::events::{AfterRegisterUserEvent, UserEventsContainer};

#[derive(Serialize, Deserialize, Clone)]
pub struct Role {
    pub name: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RoleId {
    pub value: String,
}

pub struct UserEventsListener {
    user_events: Arc<UserEventsContainer>,
}

impl UserEventsListener {
    pub(crate) fn new(user_events: Arc<UserEventsContainer>) -> UserEventsListener {
        UserEventsListener { user_events }
    }

    fn register_observer(&self, receiver: crossbeam_channel::Receiver<AfterRegisterUserEvent>) {
        thread::spawn(move || {
            while let Ok(event) = receiver.recv() {
                // Обработка события
                println!("Received event data in roles module: {}", event.user_id.value);
            }
        });
    }
}

struct Listener;

impl Listener {
    fn new() -> Self {
        Self
    }

    fn handle_event(&self, event: &AfterRegisterUserEvent) {
        // Обработка события
        println!("Received event data: {}", event.user_id.value);
    }
}