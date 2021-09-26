use super::*;

type Connection<T> = client::Connection<ServerMessage<T>, <T as Model>::Message>;

pub struct ConnectingState<T: Model, G: State> {
    geng: Geng,
    connection: Option<Pin<Box<dyn Future<Output = (T::PlayerId, T, Connection<T>)>>>>,
    f: Option<Box<dyn FnOnce(T::PlayerId, Remote<T>) -> G + 'static>>,
    transition: Option<geng::Transition>,
}

impl<T: Model, G: State> ConnectingState<T, G> {
    pub fn new(
        geng: &Geng,
        addr: &str,
        f: impl FnOnce(T::PlayerId, Remote<T>) -> G + 'static,
    ) -> Self {
        let addr = format!("{}://{}", option_env!("WSS").unwrap_or("ws"), addr);
        let connection = Box::pin(geng::net::client::connect(&addr).then(
            |connection| async move {
                let (message, connection) = connection.into_future().await;
                let player_id = match message {
                    Some(ServerMessage::PlayerId(id)) => id,
                    _ => unreachable!(),
                };
                let (message, connection) = connection.into_future().await;
                let initial_state = match message {
                    Some(ServerMessage::Full(state)) => state,
                    _ => unreachable!(),
                };
                (player_id, initial_state, connection)
            },
        ));
        Self {
            geng: geng.clone(),
            f: Some(Box::new(f)),
            connection: Some(connection),
            transition: None,
        }
    }
}

impl<T: Model, G: State> geng::State for ConnectingState<T, G> {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let framebuffer_size = framebuffer.size();
        ugli::clear(framebuffer, Some(Color::WHITE), None);
        self.geng.default_font().draw(
            framebuffer,
            &geng::PixelPerfectCamera,
            "Connecting to the server...",
            framebuffer_size.map(|x| x as f32) / 2.0,
            TextAlign::CENTER,
            40.0,
            Color::BLACK,
        );
    }
    fn update(&mut self, delta_time: f64) {}
    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::KeyDown { key, .. } => match key {
                geng::Key::Escape => {
                    self.transition = Some(geng::Transition::Pop);
                }
                _ => {}
            },
            _ => {}
        }
    }
    fn transition(&mut self) -> Option<geng::Transition> {
        if let Some(connection) = &mut self.connection {
            if let std::task::Poll::Ready((player_id, initial_state, connection)) = connection
                .as_mut()
                .poll(&mut std::task::Context::from_waker(
                    futures::task::noop_waker_ref(),
                ))
            {
                return Some(geng::Transition::Switch(Box::new(self.f.take().unwrap()(
                    player_id,
                    Remote {
                        connection: RefCell::new(connection),
                        model: RefCell::new(initial_state),
                    },
                ))));
            }
        }
        self.transition.take()
    }
}
