use super::*;

pub mod client;
#[cfg(not(target_arch = "wasm32"))]
pub mod server;
pub mod simple;
mod traffic;

#[cfg(not(target_arch = "wasm32"))]
pub use server::{Server, ServerHandle};
pub use traffic::*;

pub trait Message: Debug + Serialize + for<'de> Deserialize<'de> + Send + 'static + Unpin {}

impl<T: Debug + Serialize + for<'de> Deserialize<'de> + Send + 'static + Unpin> Message for T {}

fn serialize_message<T: Message>(message: T) -> Vec<u8> {
    bincode::serialize(&message).unwrap()
}

fn deserialize_message<T: Message>(data: &[u8]) -> T {
    bincode::deserialize(data).expect("Failed to deserialize message")
}

#[cfg(target_arch = "wasm32")]
pub trait Sender<T> {
    fn send(&mut self, message: T);
}

#[cfg(not(target_arch = "wasm32"))]
pub trait Sender<T>: Send {
    fn send(&mut self, message: T);
}

pub trait Receiver<T> {
    fn handle(&mut self, message: T);
}
