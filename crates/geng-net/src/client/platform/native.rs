use crate::Message;
use crate::Traffic;
use crate::{deserialize_message, serialize_message};
use anyhow::{anyhow, Context as _};
use futures::prelude::*;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

pub struct Connection<S: Message, C: Message> {
    sender: ws::Sender,
    broadcaster: ws::Sender,
    recv: futures::channel::mpsc::UnboundedReceiver<anyhow::Result<S>>,
    thread_handle: Option<std::thread::JoinHandle<()>>,
    phantom_data: PhantomData<(S, C)>,
    traffic: Arc<Mutex<Traffic>>,
}

impl<S: Message, C: Message> Connection<S, C> {
    pub fn traffic(&self) -> Traffic {
        self.traffic.lock().unwrap().clone()
    }
    pub fn try_recv(&mut self) -> Option<anyhow::Result<S>> {
        match self.recv.try_next() {
            Ok(Some(message)) => Some(message),
            Err(_) => None,
            Ok(None) => Some(Err(anyhow!("Disconnected from server"))),
        }
    }
    pub fn send(&mut self, message: C) {
        log::trace!("Sending message to server: {:?}", message);
        let data = serialize_message(message);
        self.traffic.lock().unwrap().outbound += data.len();
        self.sender
            .send(ws::Message::Binary(data))
            .expect("Failed to send message");
    }
}

impl<S: Message, C: Message> Stream for Connection<S, C> {
    type Item = anyhow::Result<S>;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        Stream::poll_next(unsafe { self.map_unchecked_mut(|pin| &mut pin.recv) }, cx)
    }
}

impl<S: Message, C: Message> Drop for Connection<S, C> {
    fn drop(&mut self) {
        self.broadcaster.shutdown().unwrap();
        self.thread_handle.take().unwrap().join().unwrap();
    }
}

struct Handler<T: Message> {
    connection_sender: Option<futures::channel::oneshot::Sender<ws::Sender>>,
    recv_sender: futures::channel::mpsc::UnboundedSender<anyhow::Result<T>>,
    sender: ws::Sender,
    traffic: Arc<Mutex<Traffic>>,
}

impl<T: Message> ws::Handler for Handler<T> {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        log::info!("Connected to the server");
        self.connection_sender
            .take()
            .unwrap()
            .send(self.sender.clone())
            .unwrap();
        Ok(())
    }
    fn on_message(&mut self, message: ws::Message) -> ws::Result<()> {
        let data = message.into_data();
        self.traffic.lock().unwrap().inbound += data.len();
        let message = deserialize_message(&data);
        log::trace!("Got message from server: {:?}", message);
        self.recv_sender.unbounded_send(message).unwrap();
        Ok(())
    }
}

struct Factory<T: Message> {
    connection_sender: Option<futures::channel::oneshot::Sender<ws::Sender>>,
    recv_sender: Option<futures::channel::mpsc::UnboundedSender<anyhow::Result<T>>>,
    traffic: Arc<Mutex<Traffic>>,
}

impl<T: Message> ws::Factory for Factory<T> {
    type Handler = Handler<T>;
    fn connection_made(&mut self, sender: ws::Sender) -> Handler<T> {
        Handler {
            connection_sender: self.connection_sender.take(),
            recv_sender: self.recv_sender.take().unwrap(),
            sender,
            traffic: self.traffic.clone(),
        }
    }
}

pub fn connect<S: Message, C: Message>(
    addr: &str,
) -> impl Future<Output = anyhow::Result<Connection<S, C>>> {
    let addr = addr.to_owned();
    async move {
        let (connection_sender, connection_receiver) = futures::channel::oneshot::channel();
        let (recv_sender, recv) = futures::channel::mpsc::unbounded();
        let traffic = Arc::new(Mutex::new(Traffic::new()));
        let factory = Factory {
            connection_sender: Some(connection_sender),
            recv_sender: Some(recv_sender),
            traffic: traffic.clone(),
        };
        let mut ws = ws::WebSocket::new(factory)?;
        let broadcaster = ws.broadcaster();
        ws.connect(addr.parse().context("Failed to parse addr")?)?;
        let thread_handle = std::thread::spawn(move || {
            ws.run().unwrap();
        });
        let sender = connection_receiver.await;
        Ok(Connection {
            sender: sender?,
            broadcaster,
            recv,
            thread_handle: Some(thread_handle),
            phantom_data: PhantomData,
            traffic,
        })
    }
}
