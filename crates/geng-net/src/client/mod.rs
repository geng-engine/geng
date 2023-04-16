use super::*;

mod platform;

pub struct Connection<S: Message, C: Message> {
    inner: platform::Connection<S, C>,
}

impl<S: Message, C: Message> Connection<S, C> {
    pub fn traffic(&self) -> Traffic {
        self.inner.traffic()
    }
    pub fn send(&mut self, message: C) {
        self.inner.send(message);
    }
    pub fn try_recv(&mut self) -> Option<anyhow::Result<S>> {
        self.inner.try_recv()
    }
    pub fn new_messages(&mut self) -> NewMessages<S, C> {
        NewMessages { connection: self }
    }
}

impl<S: Message, C: Message> Stream for Connection<S, C> {
    type Item = anyhow::Result<S>;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        Stream::poll_next(unsafe { self.map_unchecked_mut(|pin| &mut pin.inner) }, cx)
    }
}

pub struct NewMessages<'a, S: Message, C: Message> {
    connection: &'a mut Connection<S, C>,
}

impl<'a, S: Message, C: Message> Iterator for NewMessages<'a, S, C> {
    type Item = anyhow::Result<S>;
    fn next(&mut self) -> Option<anyhow::Result<S>> {
        self.connection.try_recv()
    }
}

impl<S: Message, C: Message> Sender<C> for Connection<S, C> {
    fn send(&mut self, message: C) {
        self.send(message);
    }
    fn send_serialized(&mut self, _data: Arc<Vec<u8>>) {
        unimplemented!()
    }
}

pub fn connect<S: Message, C: Message>(
    addr: &str,
) -> impl Future<Output = anyhow::Result<Connection<S, C>>> {
    platform::connect(addr).map(|result| result.map(|inner| Connection { inner }))
}
