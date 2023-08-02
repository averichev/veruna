use pharos::*;

#[derive(Clone, Debug, PartialEq, Copy)]
pub(crate) enum UserEvents
{
    AfterRegisterUser
}

//#[derive(Clone)]
pub struct UserEventsContainer {
    events: Pharos<UserEvents>,
}

impl UserEventsContainer {
    pub(crate) fn new() -> UserEventsContainer {
        UserEventsContainer { events: Pharos::default() }
    }
}

impl Observable<UserEvents> for UserEventsContainer
{
    type Error = PharErr;

    fn observe(&mut self, options: ObserveConfig<UserEvents>) -> Observe<'_, UserEvents, Self::Error>
    {
        self.events.observe(options)
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