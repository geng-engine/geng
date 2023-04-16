use super::*;

struct ClientState<T: Model> {
    sender: Box<dyn geng::net::Sender<ServerMessage<T>>>,
}

struct ServerState<T: Model> {
    current: T,
    previous: T,
    events: Vec<T::Event>,
    next_client_id: usize,
    clients: HashMap<usize, ClientState<T>>,
}

impl<T: Model> ServerState<T> {
    fn send_updates(&mut self) {
        if self.current != self.previous {
            let delta = self.previous.diff(&self.current);
            self.previous = self.current.clone();
            for client in self.clients.values_mut() {
                client.sender.send(ServerMessage::Delta(delta.clone()));
            }
        }
        let events = std::mem::take(&mut self.events);
        if !events.is_empty() {
            for client in self.clients.values_mut() {
                client.sender.send(ServerMessage::Events(events.clone()));
            }
        }
    }
}

struct Client<T: Model> {
    player_id: T::PlayerId,
    client_id: usize,
    server_state: Arc<Mutex<ServerState<T>>>,
}

impl<T: Model> geng::net::Receiver<T::Message> for Client<T> {
    fn handle(&mut self, message: T::Message) {
        let mut state = self.server_state.lock().unwrap();
        let state: &mut ServerState<T> = &mut state;
        let replies = state
            .current
            .handle_message(&mut state.events, &self.player_id, message);
        if !replies.is_empty() {
            state
                .clients
                .get_mut(&self.client_id)
                .unwrap()
                .sender
                .send(ServerMessage::Events(replies));
        }
    }
}

impl<T: Model> Drop for Client<T> {
    fn drop(&mut self) {
        let mut state = self.server_state.lock().unwrap();
        let state: &mut ServerState<T> = &mut state;
        state
            .current
            .drop_player(&mut state.events, &self.player_id);
        state.clients.remove(&self.client_id);
    }
}

struct ServerApp<T: Model> {
    state: Arc<Mutex<ServerState<T>>>,
}

pub struct Server<T: Model> {
    state: Arc<Mutex<ServerState<T>>>,
    inner: geng::net::Server<ServerApp<T>>,
}

impl<T: Model> Server<T> {
    pub fn new<A: std::net::ToSocketAddrs + Debug + Copy>(addr: A, model: T) -> Self {
        let state = Arc::new(Mutex::new(ServerState {
            current: model.clone(),
            previous: model,
            events: Vec::new(),
            next_client_id: 0,
            clients: HashMap::new(),
        }));
        Self {
            state: state.clone(),
            inner: geng::net::Server::new(ServerApp { state }, addr),
        }
    }
    pub fn handle(&self) -> geng::net::ServerHandle {
        self.inner.handle()
    }
    pub fn run(self) {
        let running = Arc::new(std::sync::atomic::AtomicBool::new(true));
        let server_thread = std::thread::spawn({
            let state = self.state;
            let running = running.clone();
            let mut timer = Timer::new();
            let mut unprocessed_time: f32 = 0.0;
            move || {
                while running.load(std::sync::atomic::Ordering::Relaxed) {
                    unprocessed_time += timer.tick().as_secs_f64() as f32;
                    unprocessed_time = unprocessed_time.min(1.0);
                    {
                        let mut state = state.lock().unwrap();
                        let state: &mut ServerState<T> = &mut state;
                        while unprocessed_time > 1.0 / T::TICKS_PER_SECOND {
                            unprocessed_time -= 1.0 / T::TICKS_PER_SECOND;
                            state.current.tick(&mut state.events);
                        }
                        state.send_updates();
                    }
                    std::thread::sleep(std::time::Duration::from_secs_f32(
                        1.0 / T::TICKS_PER_SECOND - unprocessed_time,
                    ));
                }
            }
        });
        self.inner.run();
        running.store(false, std::sync::atomic::Ordering::Relaxed);
        server_thread.join().expect("Failed to join server thread");
    }
}

impl<T: Model> geng::net::server::App for ServerApp<T> {
    type Client = Client<T>;
    type ServerMessage = ServerMessage<T>;
    type ClientMessage = T::Message;
    fn connect(&mut self, mut sender: Box<dyn geng::net::Sender<ServerMessage<T>>>) -> Client<T> {
        let mut state = self.state.lock().unwrap();
        let state: &mut ServerState<T> = &mut state;
        let player_id = state.current.new_player(&mut state.events);
        sender.send(ServerMessage::PlayerId(player_id.clone()));
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
