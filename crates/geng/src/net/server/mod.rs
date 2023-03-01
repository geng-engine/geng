use super::*;

pub trait App: Send + 'static {
    type Client: Receiver<Self::ClientMessage>;
    type ServerMessage: Message;
    type ClientMessage: Message;
    fn connect(&mut self, sender: Box<dyn Sender<Self::ServerMessage>>) -> Self::Client;
}

struct Handler<T: App> {
    app: Arc<Mutex<T>>,
    sender: ws::Sender,
    client: Option<T::Client>,
}

struct BackgroundSender {
    sender: std::sync::mpsc::Sender<Arc<Vec<u8>>>,
}

impl BackgroundSender {
    fn new(ws_sender: ws::Sender) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel::<Arc<Vec<u8>>>();
        std::thread::spawn(move || {
            while let Ok(data) = receiver.recv() {
                ws_sender
                    .send(ws::Message::Binary(data.deref().clone()))
                    .expect("Failed to send message");
            }
        });
        Self { sender }
    }
}

impl<T: Message> Sender<T> for BackgroundSender {
    fn send_serialized(&mut self, data: Arc<Vec<u8>>) {
        self.sender.send(data).expect("Failed to send message");
    }
}

impl<T: App> ws::Handler for Handler<T> {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        self.client = Some(
            self.app
                .lock()
                .unwrap()
                .connect(Box::new(BackgroundSender::new(self.sender.clone()))),
        );
        Ok(())
    }
    fn on_message(&mut self, message: ws::Message) -> ws::Result<()> {
        let message = match deserialize_message(&message.into_data()) {
            Ok(message) => message,
            Err(e) => {
                return Err(ws::Error::new(ws::ErrorKind::Protocol, e.to_string()));
            }
        };
        trace!("Received message from client: {:?}", message);
        if let Some(client) = &mut self.client {
            client.handle(message);
        } else {
            error!("WUT! received a message before handshake");
        }
        Ok(())
    }
}

struct Factory<T: App> {
    app: Arc<Mutex<T>>,
}

impl<T: App> Factory<T> {
    fn new(app: T) -> Self {
        Self {
            app: Arc::new(Mutex::new(app)),
        }
    }
}

impl<T: App> ws::Factory for Factory<T> {
    type Handler = Handler<T>;

    fn connection_made(&mut self, sender: ws::Sender) -> Handler<T> {
        info!("New connection");
        Handler {
            app: self.app.clone(),
            sender,
            client: None,
        }
    }
}

pub struct Server<T: App> {
    ws: ws::WebSocket<Factory<T>>,
}

#[derive(Clone)]
pub struct ServerHandle {
    sender: ws::Sender,
}

impl ServerHandle {
    pub fn shutdown(&self) {
        self.sender.shutdown().expect("Failed to shutdown server");
    }
}

impl<T: App> Server<T> {
    pub fn new(app: T, addr: impl std::net::ToSocketAddrs + Debug + Copy) -> Self {
        let factory = Factory::new(app);
        let ws = ws::Builder::new()
            .with_settings(ws::Settings {
                max_connections: 10000,
                // fragments_capacity: todo!(),
                // fragments_grow: todo!(),
                // max_fragment_size: todo!(),
                // in_buffer_capacity: todo!(),
                // in_buffer_grow: todo!(),
                // out_buffer_capacity: todo!(),
                // out_buffer_grow: todo!(),
                panic_on_internal: false,
                tcp_nodelay: true,
                ..default()
            })
            .build(factory)
            .unwrap();
        let ws = match ws.bind(addr) {
            Ok(ws) => ws,
            Err(e) => {
                error!("Failed to bind server to {:?}: {}", addr, e);
                panic!("{e:?}");
            }
        };
        Self { ws }
    }
    pub fn handle(&self) -> ServerHandle {
        ServerHandle {
            sender: self.ws.broadcaster(),
        }
    }
    pub fn run(self) {
        info!("Starting the server");
        match self.ws.run() {
            Ok(_) => {
                info!("Server finished successfully");
            }
            Err(e) => {
                error!("Server shutdown with error: {}", e);
                panic!("{e:?}");
            }
        }
    }
}
