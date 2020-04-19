use crate::*;

pub struct Connection<S: Message, C: Message> {
    ws: stdweb::web::WebSocket,
    recv: futures::channel::mpsc::UnboundedReceiver<S>,
    phantom_data: PhantomData<(S, C)>,
    traffic: Arc<Traffic>,
}

impl<S: Message, C: Message> Connection<S, C> {
    pub fn traffic(&self) -> &Traffic {
        &self.traffic
    }
    pub fn try_recv(&mut self) -> Option<S> {
        match self.recv.try_next() {
            Ok(Some(message)) => Some(message),
            Err(_) => None,
            Ok(None) => panic!("Disconnected from server"),
        }
    }
    pub fn send(&mut self, message: C) {
        let data = serialize_message(message);
        self.traffic.add_outbound(data.len());
        self.ws.send_bytes(&data).expect("Failed to send message");
    }
}

impl<S: Message, C: Message> Stream for Connection<S, C> {
    type Item = S;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        Stream::poll_next(unsafe { self.map_unchecked_mut(|pin| &mut pin.recv) }, cx)
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
    let (recv_sender, recv) = futures::channel::mpsc::unbounded();
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
        recv_sender.unbounded_send(message).unwrap();
    });
    connection_receiver.map(|result| result.unwrap())
}
