use std::sync::{Arc};
use std::thread;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use crate::DataErrorTrait;
use crate::users::events::{AfterRegisterUserEvent, UserEventsContainer};
use crate::users::UsersRepository;


#[async_trait(? Send)]
pub trait RolesRepository: Send {
    async fn get_role_id(&self, role_name: String) -> Result<Option<RoleId>, Box<dyn DataErrorTrait>>;
}

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
    user_repository: Arc<Mutex<dyn UsersRepository>>,
    roles_repository: Arc<Mutex<dyn RolesRepository>>,
}

impl UserEventsListener {
    pub(crate) fn new(user_events: Arc<UserEventsContainer>, user_repository: Arc<Mutex<dyn UsersRepository>>, roles_repository: Arc<Mutex<dyn RolesRepository>>) -> UserEventsListener {
        UserEventsListener { user_events, user_repository, roles_repository }
    }

    async fn handle_event(&self, event: AfterRegisterUserEvent) {
        let mut repository = self.user_repository.lock().await;
        let roles_repository = self.roles_repository.lock().await;
        let count = repository.count_users().await.unwrap();
        if count == 1u32 {
            let role_id = roles_repository.get_role_id(String::from("admin")).await.unwrap().unwrap();
            repository.add_user_role(event.user_id, role_id).await.unwrap();
        }
    }

    pub fn register_observer(self, receiver: crossbeam_channel::Receiver<AfterRegisterUserEvent>) {
        thread::spawn(move || {
            let task = async move {
                while let Ok(event) = receiver.recv() {
                    self.handle_event(event).await;
                }
            };
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(task);
        });
    }
}