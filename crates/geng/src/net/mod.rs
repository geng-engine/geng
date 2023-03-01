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

pub fn serialize_message<T: Message>(message: T) -> Vec<u8> {
    let mut buf = Vec::new();
    let writer = flate2::write::GzEncoder::new(&mut buf, flate2::Compression::best());
    bincode::serialize_into(writer, &message).unwrap();
    buf
}

pub fn deserialize_message<T: Message>(data: &[u8]) -> anyhow::Result<T> {
    let reader = flate2::read::GzDecoder::new(data);
    bincode::deserialize_from(reader).context("Failed to deserialize message")
}

#[cfg(target_arch = "wasm32")]
pub trait Sender<T> {
    fn send(&mut self, message: T)
    where
        T: Message,
    {
        self.send_serialized(Arc::new(serialize_message(message)))
    }
    fn send_serialized(&mut self, data: Arc<Vec<u8>>);
}

#[cfg(not(target_arch = "wasm32"))]
pub trait Sender<T>: Send {
    fn send(&mut self, message: T)
    where
        T: Message,
    {
        self.send_serialized(Arc::new(serialize_message(message)))
    }
    fn send_serialized(&mut self, data: Arc<Vec<u8>>);
}

pub trait Receiver<T> {
    fn handle(&mut self, message: T);
}
