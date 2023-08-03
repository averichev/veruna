use crate::users::user_id::UserId;

pub struct AfterRegisterUserEvent {
    pub(crate) user_id: UserId,
}

pub enum UsrEvents
{
    AfterRegister(AfterRegisterUserEvent)
}

pub struct UserEventsNotifier {
    event_type: UsrEvents,
}

impl UserEventsNotifier {
    pub fn new(event_type: UsrEvents) -> UserEventsNotifier {
        UserEventsNotifier { event_type }
    }
}

#[derive(Clone)]
pub struct UserEventsContainer {
    sender: crossbeam_channel::Sender<AfterRegisterUserEvent>,
    pub test_data: String
}

impl UserEventsContainer {
    pub(crate) fn new() -> UserEventsContainer {
        println!("UserEventsContainer new");
        let (sender, _) = crossbeam_channel::unbounded::<AfterRegisterUserEvent>();
        UserEventsContainer { sender, test_data: "Not empty string".to_string() }
    }

    pub(crate) fn notify(&self, event: UsrEvents) {
        match event {
            UsrEvents::AfterRegister(after_register) => {
               let res = self.sender.send(after_register);
                match res {
                    Ok(ok) => {
                        println!("Сообщение успешно отправлено")
                    }
                    Err(e) => {
                        println!("{}", e)
                    }
                }
            }
        }
    }
}

// #[derive(Clone)]
// pub struct UserEventsContainer {
//     pub after_register_user: Pharos<AfterRegisterUserEvent>,
//
// }
//
// impl UserEventsContainer {
//     pub(crate) fn new() -> UserEventsContainer {
//         UserEventsContainer { after_register_user: Pharos::default() }
//     }
//     pub async fn sail(&mut self)
//     {
//         self.pharos.send(AfterRegisterUserEvent::Sailing).await.expect("notify observers");
//     }
// }
//
// #[derive(Clone, Debug, PartialEq, Copy)]
// enum AfterRegisterUserEvent
// {
//     Sailing
// }
//
// impl Observable<AfterRegisterUserEvent> for UserEventsContainer
// {
//     type Error = PharErr;
//
//     fn observe(&mut self, options: ObserveConfig<AfterRegisterUserEvent>) -> Observe<'_, AfterRegisterUserEvent, Self::Error>
//     {
//         self.pharos.observe(options)
//     }
// }