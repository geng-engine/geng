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
    let mut buf = Vec::new();
    let writer = flate2::write::GzEncoder::new(&mut buf, flate2::Compression::best());
    bincode::serialize_into(writer, &message).unwrap();
    buf
}

fn deserialize_message<T: Message>(data: &[u8]) -> T {
    let reader = flate2::read::GzDecoder::new(data);
    bincode::deserialize_from(reader).expect("Failed to deserialize message")
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
