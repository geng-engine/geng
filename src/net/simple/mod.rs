use super::*;

mod app;
mod lobby;

pub use app::run;
pub use lobby::*;
#[cfg(not(target_arch = "wasm32"))]
pub mod server;
#[cfg(not(target_arch = "wasm32"))]
pub use server::*;

pub trait Model: Diff + Message {
    type PlayerId: Message + Clone;
    type Message: Message;
    const TICKS_PER_SECOND: f32;
    fn new_player(&mut self) -> Self::PlayerId;
    fn drop_player(&mut self, player_id: &Self::PlayerId);
    fn handle_message(&mut self, player_id: &Self::PlayerId, message: Self::Message);
    fn tick(&mut self);
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage<T: Model> {
    PlayerId(T::PlayerId),
    Delta(#[serde(bound = "")] <T as Diff>::Delta),
    Full(#[serde(bound = "")] T),
}

pub struct Remote<T: Model> {
    connection: RefCell<client::Connection<ServerMessage<T>, T::Message>>,
    model: RefCell<T>,
}

impl<T: Model> Remote<T> {
    fn update(&self) {
        let mut model = self.model.borrow_mut();
        for message in self.connection.borrow_mut().new_messages() {
            match message {
                ServerMessage::Full(state) => *model = state,
                ServerMessage::Delta(delta) => model.update(&delta),
                ServerMessage::PlayerId(_) => unreachable!(),
            }
        }
    }
    pub fn get(&self) -> Ref<T> {
        self.update();
        self.model.borrow()
    }
    pub fn send(&self, message: T::Message) {
        self.connection.borrow_mut().send(message);
    }
}
