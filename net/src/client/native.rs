use crate::*;

pub struct Connection<S: Message, C: Message> {
    sender: ws::Sender,
    broadcaster: ws::Sender,
    recv: std::sync::mpsc::Receiver<S>,
    thread_handle: Option<std::thread::JoinHandle<()>>,
    phantom_data: PhantomData<(S, C)>,
    traffic: Arc<Traffic>,
}

impl<S: Message, C: Message> Connection<S, C> {
    pub fn traffic(&self) -> &Traffic {
        &self.traffic
    }
    pub fn try_recv(&mut self) -> Option<S> {
        match self.recv.try_recv() {
            Ok(message) => Some(message),
            Err(std::sync::mpsc::TryRecvError::Empty) => None,
            Err(std::sync::mpsc::TryRecvError::Disconnected) => panic!("Disconnected from server"),
        }
    }
    pub fn send(&mut self, message: C) {
        trace!("Sending message to server: {:?}", message);
        let data = serialize_message(message);
        self.traffic.add_outbound(data.len());
        self.sender
            .send(ws::Message::Binary(data))
            .expect("Failed to send message");
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
    recv_sender: std::sync::mpsc::Sender<T>,
    sender: ws::Sender,
    traffic: Arc<Traffic>,
}

impl<T: Message> ws::Handler for Handler<T> {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        info!("Connected to the server");
        self.connection_sender
            .take()
            .unwrap()
            .send(self.sender.clone())
            .unwrap();
        Ok(())
    }
    fn on_message(&mut self, message: ws::Message) -> ws::Result<()> {
        let data = message.into_data();
        self.traffic.add_inbound(data.len());
        let message = deserialize_message(&data);
        trace!("Got message from server: {:?}", message);
        self.recv_sender.send(message).unwrap();
        Ok(())
    }
}

struct Factory<T: Message> {
    connection_sender: Option<futures::channel::oneshot::Sender<ws::Sender>>,
    recv_sender: Option<std::sync::mpsc::Sender<T>>,
    traffic: Arc<Traffic>,
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

pub fn connect<S: Message, C: Message>(addr: &str) -> impl Future<Output = Connection<S, C>> {
    let (connection_sender, connection_receiver) = futures::channel::oneshot::channel();
    let (recv_sender, recv) = std::sync::mpsc::channel();
    let traffic = Arc::new(Traffic::new());
    let factory = Factory {
        connection_sender: Some(connection_sender),
        recv_sender: Some(recv_sender),
        traffic: traffic.clone(),
    };
    let mut ws = ws::WebSocket::new(factory).unwrap();
    let mut broadcaster = Some(ws.broadcaster());
    ws.connect(addr.parse().unwrap()).unwrap();
    let mut thread_handle = Some(std::thread::spawn(move || {
        ws.run().unwrap();
    }));
    let mut recv = Some(recv);
    connection_receiver.map(move |sender| Connection {
        sender: sender.unwrap(),
        broadcaster: broadcaster.take().unwrap(),
        recv: recv.take().unwrap(),
        thread_handle: thread_handle.take(),
        phantom_data: PhantomData,
        traffic,
    })
}
