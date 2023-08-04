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
    receiver: crossbeam_channel::Receiver<AfterRegisterUserEvent>,
    pub test_data: String
}

impl UserEventsContainer {
    pub(crate) fn new() -> UserEventsContainer {
        println!("UserEventsContainer new");
        let (sender, receiver) = crossbeam_channel::unbounded::<AfterRegisterUserEvent>();
        UserEventsContainer { sender, receiver, test_data: "Not empty string".to_string() }
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

    pub fn receiver(&self) -> crossbeam_channel::Receiver<AfterRegisterUserEvent>{
        self.receiver.clone()
    }
}