use crate::{deserialize_message, serialize_message, Message, Traffic};
use anyhow::anyhow;
use futures::prelude::*;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use wasm_bindgen::prelude::*;

pub struct Connection<S: Message, C: Message> {
    ws: web_sys::WebSocket,
    recv: futures::channel::mpsc::UnboundedReceiver<anyhow::Result<S>>,
    phantom_data: PhantomData<(S, C)>,
    traffic: Arc<Mutex<Traffic>>,
}

impl<S: Message, C: Message> Connection<S, C> {
    pub fn traffic(&self) -> Traffic {
        self.traffic.lock().unwrap().clone()
    }
    pub fn try_recv(&mut self) -> Option<anyhow::Result<S>> {
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
    type Item = anyhow::Result<S>;
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

pub fn connect<S: Message, C: Message>(
    addr: &str,
) -> impl Future<Output = anyhow::Result<Connection<S, C>>> {
    let ws = web_sys::WebSocket::new(addr).unwrap();
    let (mut connection_sender, connection_receiver) =
        futures::channel::mpsc::channel::<anyhow::Result<Connection<S, C>>>(1);
    let (recv_sender, recv) = futures::channel::mpsc::unbounded();
    let traffic = Arc::new(Mutex::new(Traffic::new()));
    let connection = Connection {
        ws: ws.clone(),
        phantom_data: PhantomData,
        recv,
        traffic: traffic.clone(),
    };
    let connection_error_listener = wasm_bindgen::closure::Closure::once_into_js(Box::new({
        let mut connection_sender = connection_sender.clone();
        move || {
            assert!(connection_sender
                .try_send(Err(anyhow!("Failed to connect")))
                .is_ok());
        }
    })
        as Box<dyn FnOnce()>);
    ws.add_event_listener_with_callback(
        "open",
        wasm_bindgen::closure::Closure::once_into_js(Box::new({
            let ws = ws.clone();
            let connection_error_listener = connection_error_listener.clone();
            move || {
                assert!(connection_sender.try_send(Ok(connection)).is_ok());

                ws.remove_event_listener_with_callback(
                    "error",
                    connection_error_listener.unchecked_ref(),
                )
                .unwrap();
                ws.add_event_listener_with_callback(
                    "error",
                    wasm_bindgen::closure::Closure::once_into_js(Box::new({
                        let recv_sender = recv_sender.clone();
                        move || {
                            recv_sender
                                .unbounded_send(Err(anyhow!("WebSocket error")))
                                .unwrap();
                        }
                    })
                        as Box<dyn FnOnce()>)
                    .unchecked_ref(),
                )
                .unwrap();
                ws.add_event_listener_with_callback(
                    "close",
                    wasm_bindgen::closure::Closure::once_into_js(Box::new({
                        let recv_sender = recv_sender.clone();
                        move || {
                            recv_sender
                                .unbounded_send(Err(anyhow!("Connection closed")))
                                .unwrap();
                        }
                    })
                        as Box<dyn FnOnce()>)
                    .unchecked_ref(),
                )
                .unwrap();

                let message_handler = wasm_bindgen::closure::Closure::wrap(Box::new(
                    move |event: web_sys::MessageEvent| {
                        let data: Vec<u8> = js_sys::Uint8Array::new(
                            event
                                .data()
                                .dyn_into::<js_sys::ArrayBuffer>()
                                .unwrap()
                                .as_ref(),
                        )
                        .to_vec();
                        traffic.lock().unwrap().inbound += data.len();
                        let message = deserialize_message(&data).unwrap();
                        recv_sender.unbounded_send(Ok(message)).unwrap();
                    },
                )
                    as Box<dyn FnMut(web_sys::MessageEvent)>);
                ws.add_event_listener_with_callback(
                    "message",
                    message_handler.into_js_value().unchecked_ref(),
                )
                .unwrap();
            }
        }) as Box<dyn FnOnce()>)
        .unchecked_ref(),
    )
    .unwrap();
    ws.add_event_listener_with_callback("error", connection_error_listener.unchecked_ref())
        .unwrap();
    ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

    connection_receiver
        .into_future()
        .map(|(result, _)| result.unwrap())
}
