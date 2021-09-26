use super::*;

struct ClientState<T: Model> {
    sender: Box<dyn Sender<ServerMessage<T>>>,
}

struct ServerState<T: Model> {
    current: T,
    previous: T,
    next_client_id: usize,
    clients: HashMap<usize, ClientState<T>>,
}

impl<T: Model> ServerState<T> {
    fn update(&mut self) {
        if self.current != self.previous {
            let delta = self.previous.diff(&self.current);
            self.previous = self.current.clone();
            for client in self.clients.values_mut() {
                client.sender.send(ServerMessage::Delta(delta.clone()));
            }
        }
    }
}

struct Client<T: Model> {
    player_id: T::PlayerId,
    client_id: usize,
    server_state: Arc<Mutex<ServerState<T>>>,
}

impl<T: Model> Receiver<T::Message> for Client<T> {
    fn handle(&mut self, message: T::Message) {
        self.server_state
            .lock()
            .unwrap()
            .current
            .handle_message(&self.player_id, message);
    }
}

impl<T: Model> Drop for Client<T> {
    fn drop(&mut self) {
        let mut state = self.server_state.lock().unwrap();
        state.current.drop_player(&self.player_id);
        state.clients.remove(&self.client_id);
    }
}

struct ServerApp<T: Model> {
    state: Arc<Mutex<ServerState<T>>>,
}

pub struct Server<T: Model> {
    state: Arc<Mutex<ServerState<T>>>,
    inner: net::Server<ServerApp<T>>,
}

impl<T: Model> Server<T> {
    pub fn new<A: std::net::ToSocketAddrs + Debug + Copy>(addr: A, model: T) -> Self {
        let state = Arc::new(Mutex::new(ServerState {
            current: model.clone(),
            previous: model.clone(),
            next_client_id: 0,
            clients: HashMap::new(),
        }));
        Self {
            state: state.clone(),
            inner: net::Server::new(
                ServerApp {
                    state: state.clone(),
                },
                addr,
            ),
        }
    }
    pub fn handle(&self) -> ServerHandle {
        self.inner.handle()
    }
    pub fn run(self) {
        let running = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let server_thread = std::thread::spawn({
            let state = self.state;
            let running = running.clone();
            let mut sleep_time = 0.0;
            move || {
                while running.load(std::sync::atomic::Ordering::Relaxed) {
                    // TODO: smoother TPS
                    std::thread::sleep(std::time::Duration::from_secs_f32(sleep_time));
                    {
                        let mut state = state.lock().unwrap();
                        state.current.tick();
                        state.update();
                    }
                    sleep_time = 1.0 / T::TICKS_PER_SECOND;
                }
            }
        });
        self.inner.run();
        running.store(false, std::sync::atomic::Ordering::Relaxed);
        server_thread.join().expect("Failed to join server thread");
    }
}

impl<T: Model> net::server::App for ServerApp<T> {
    type Client = Client<T>;
    type ServerMessage = ServerMessage<T>;
    type ClientMessage = T::Message;
    fn connect(&mut self, mut sender: Box<dyn Sender<ServerMessage<T>>>) -> Client<T> {
        let mut state = self.state.lock().unwrap();
        let state = &mut *state;
        let player_id = state.current.new_player();
        sender.send(ServerMessage::PlayerId(player_id.clone()));
        state.update();
        sender.send(ServerMessage::Full(state.current.clone()));
        let client_id = state.next_client_id;
        state.clients.insert(client_id, ClientState { sender });
        state.next_client_id += 1;
        Client {
            player_id,
            client_id,
            server_state: self.state.clone(),
        }
    }
}
