use super::*;

type Connection<T> = geng::net::client::Connection<ServerMessage<T>, <T as Model>::Message>;

pub struct ConnectingState<T: Model, G: geng::State> {
    geng: Geng,
    #[allow(clippy::type_complexity)]
    connection: Option<Pin<Box<dyn Future<Output = (T::PlayerId, T, Connection<T>)>>>>,
    #[allow(clippy::type_complexity)]
    f: Option<Box<dyn FnOnce(T::PlayerId, Remote<T>) -> G + 'static>>,
    transition: Option<geng::state::Transition>,
}

impl<T: Model, G: geng::State> ConnectingState<T, G> {
    pub fn new(
        geng: &Geng,
        addr: &str,
        f: impl FnOnce(T::PlayerId, Remote<T>) -> G + 'static,
    ) -> Self {
        let connection = Box::pin(
            geng::net::client::connect(addr).then(|connection| async move {
                let connection = connection.unwrap();
                let (message, connection) = connection.into_future().await;
                let player_id = match message.unwrap().unwrap() {
                    ServerMessage::PlayerId(id) => id,
                    _ => unreachable!(),
                };
                let (message, connection) = connection.into_future().await;
                let initial_state = match message.unwrap().unwrap() {
                    ServerMessage::Full(state) => state,
                    _ => unreachable!(),
                };
                (player_id, initial_state, connection)
            }),
        );
        Self {
            geng: geng.clone(),
            f: Some(Box::new(f)),
            connection: Some(connection),
            transition: None,
        }
    }
}

impl<T: Model, G: geng::State> geng::State for ConnectingState<T, G> {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let framebuffer_size = framebuffer.size();
        ugli::clear(framebuffer, Some(Rgba::WHITE), None, None);
        self.geng.default_font().draw(
            framebuffer,
            &geng::PixelPerfectCamera,
            "Connecting to the server...",
            vec2::splat(geng::TextAlign::CENTER),
            mat3::translate(framebuffer_size.map(|x| x as f32) / 2.0) * mat3::scale_uniform(40.0),
            Rgba::BLACK,
        );
    }
    fn handle_event(&mut self, event: geng::Event) {
        if matches!(
            event,
            geng::Event::KeyDown {
                key: geng::Key::Escape
            }
        ) {
            self.transition = Some(geng::state::Transition::Pop);
        }
    }
    fn transition(&mut self) -> Option<geng::state::Transition> {
        if let Some(connection) = &mut self.connection {
            if let std::task::Poll::Ready((player_id, initial_state, connection)) = connection
                .as_mut()
                .poll(&mut std::task::Context::from_waker(
                    futures::task::noop_waker_ref(),
                ))
            {
                return Some(geng::state::Transition::Switch(Box::new(self
                    .f
                    .take()
                    .unwrap()(
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
