use crate::*;

#[cfg(target_arch = "wasm32")]
#[path = "web.rs"]
mod _impl;

#[cfg(not(target_arch = "wasm32"))]
#[path = "native.rs"]
mod _impl;

pub struct Connection<S: Message, C: Message> {
    inner: _impl::Connection<S, C>,
}

impl<S: Message, C: Message> Connection<S, C> {
    pub fn traffic(&self) -> &Traffic {
        self.inner.traffic()
    }
    pub fn send(&mut self, message: C) {
        self.inner.send(message);
    }
    pub fn try_recv(&mut self) -> Option<S> {
        self.inner.try_recv()
    }
    pub fn new_messages(&mut self) -> NewMessages<S, C> {
        NewMessages { connection: self }
    }
}

pub struct NewMessages<'a, S: Message, C: Message> {
    connection: &'a mut Connection<S, C>,
}

impl<'a, S: Message, C: Message> Iterator for NewMessages<'a, S, C> {
    type Item = S;
    fn next(&mut self) -> Option<S> {
        self.connection.try_recv()
    }
}

impl<S: Message, C: Message> Sender<C> for Connection<S, C> {
    fn send(&mut self, message: C) {
        self.send(message);
    }
}

pub fn connect<S: Message, C: Message>(addr: &str) -> impl Future<Output = Connection<S, C>> {
    _impl::connect(addr).map(|inner| Connection { inner })
}
