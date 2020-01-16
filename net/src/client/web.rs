use crate::*;

pub struct Connection<S: Message, C: Message> {
    ws: stdweb::web::WebSocket,
    recv: std::sync::mpsc::Receiver<S>,
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
        let data = serialize_message(message);
        self.traffic.add_outbound(data.len());
        self.ws.send_bytes(&data).expect("Failed to send message");
    }
}

impl<S: Message, C: Message> Drop for Connection<S, C> {
    fn drop(&mut self) {
        self.ws.close();
    }
}

pub fn connect<S: Message, C: Message>(addr: &str) -> impl Future<Output = Connection<S, C>> {
    let ws = stdweb::web::WebSocket::new(addr).unwrap();
    let (connection_sender, connection_receiver) = futures::channel::oneshot::channel();
    let (recv_sender, recv) = std::sync::mpsc::channel();
    let traffic = Arc::new(Traffic::new());
    let connection = Connection {
        ws: ws.clone(),
        phantom_data: PhantomData,
        recv,
        traffic: traffic.clone(),
    };
    let mut connection_sender = Some(connection_sender);
    let mut connection = Some(connection);
    use stdweb::web::IEventTarget;
    ws.add_event_listener(move |_: stdweb::web::event::SocketOpenEvent| {
        assert!(connection_sender
            .take()
            .unwrap()
            .send(connection.take().unwrap())
            .is_ok(),);
    });
    ws.set_binary_type(stdweb::web::SocketBinaryType::ArrayBuffer);
    ws.add_event_listener(move |event: stdweb::web::event::SocketMessageEvent| {
        use stdweb::web::event::IMessageEvent;
        let data: Vec<u8> = event.data().into_array_buffer().unwrap().into();
        traffic.add_inbound(data.len());
        let message = deserialize_message(&data);
        recv_sender.send(message).unwrap();
    });
    connection_receiver.map(|result| result.unwrap())
}
