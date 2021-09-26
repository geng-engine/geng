use super::*;

pub struct Connection<S: Message, C: Message> {
    ws: web_sys::WebSocket,
    recv: futures::channel::mpsc::UnboundedReceiver<S>,
    phantom_data: PhantomData<(S, C)>,
    traffic: Arc<Mutex<Traffic>>,
}

impl<S: Message, C: Message> Connection<S, C> {
    pub fn traffic(&self) -> Traffic {
        self.traffic.lock().unwrap().clone()
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
        self.traffic.lock().unwrap().outbound += data.len();
        self.ws
            .send_with_u8_array(&data)
            .expect("Failed to send message");
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
        self.ws.close().unwrap();
    }
}

pub fn connect<S: Message, C: Message>(addr: &str) -> impl Future<Output = Connection<S, C>> {
    let ws = web_sys::WebSocket::new(addr).unwrap();
    let (connection_sender, connection_receiver) = futures::channel::oneshot::channel();
    let (recv_sender, recv) = futures::channel::mpsc::unbounded();
    let traffic = Arc::new(Mutex::new(Traffic::new()));
    let connection = Connection {
        ws: ws.clone(),
        phantom_data: PhantomData,
        recv,
        traffic: traffic.clone(),
    };
    let mut connection_sender = Some(connection_sender);
    let mut connection = Some(connection);
    ws.add_event_listener_with_callback(
        "open",
        wasm_bindgen::closure::Closure::once_into_js(Box::new(move || {
            assert!(connection_sender
                .take()
                .unwrap()
                .send(connection.take().unwrap())
                .is_ok());
        }) as Box<dyn FnOnce()>)
        .unchecked_ref(),
    )
    .unwrap();
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    let message_handler =
        wasm_bindgen::closure::Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
            let data: Vec<u8> = js_sys::Uint8Array::new(
                event
                    .data()
                    .dyn_into::<js_sys::ArrayBuffer>()
                    .unwrap()
                    .as_ref(),
            )
            .to_vec();
            traffic.lock().unwrap().inbound += data.len();
            let message = deserialize_message(&data);
            recv_sender.unbounded_send(message).unwrap();
        }) as Box<dyn FnMut(web_sys::MessageEvent)>);
    ws.add_event_listener_with_callback("message", message_handler.as_ref().unchecked_ref())
        .unwrap();
    message_handler.forget(); // TODO not forget

    connection_receiver.map(|result| result.unwrap())
}
