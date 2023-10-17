use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

const PORT: u16 = 7357;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Message {
    A,
    B,
    C,
}

mod server {
    use super::*;

    pub type Sender = Box<dyn geng_net::Sender<Message>>;

    pub struct Fns {
        pub client_connect: Arc<dyn Fn(&mut Sender) + Sync + Send>,
        pub client_drop: Arc<dyn Fn(&mut Sender) + Sync + Send>,
    }

    pub struct TestApp {
        fns: Fns,
    }

    pub struct Client {
        sender: Sender,
        drop_fn: Arc<dyn Fn(&mut Sender) + Sync + Send>,
    }

    impl Drop for Client {
        fn drop(&mut self) {
            (self.drop_fn)(&mut self.sender)
        }
    }

    impl geng_net::Receiver<Message> for Client {
        fn handle(&mut self, _message: Message) {
            unreachable!()
        }
    }

    impl geng_net::server::App for TestApp {
        type Client = Client;
        type ServerMessage = Message;
        type ClientMessage = Message;
        fn connect(&mut self, mut sender: Sender) -> Client {
            (self.fns.client_connect)(&mut sender);
            Client {
                sender,
                drop_fn: self.fns.client_drop.clone(),
            }
        }
    }

    pub fn new(fns: Fns) -> geng_net::Server<TestApp> {
        geng_net::Server::new(TestApp { fns }, ("localhost", PORT))
    }
}

#[test]
fn main() {
    let server = server::new(server::Fns {
        client_connect: Arc::new(|sender| {
            sender.send(Message::A);
            println!("connect");
        }),
        client_drop: Arc::new(|sender| {
            sender.send(Message::C);
            println!("drop");
        }),
    });
    let server_handle = server.handle();
    let server_thread = std::thread::spawn(|| server.run());

    futures::executor::block_on(async {
        let mut client =
            geng_net::client::connect::<Message, Message>(&format!("ws://localhost:{PORT}"))
                .await
                .unwrap();
        assert_eq!(client.next().await.unwrap().unwrap(), Message::A);
        client.send(Message::B);
    });

    server_handle.shutdown();
    server_thread.join().unwrap();
}
