use super::*;

use std::sync::Mutex;

struct Record {
    message: String,
}

struct Logger {
    sender: Mutex<std::sync::mpsc::Sender<Record>>,
}

impl Logger {
    fn new(sender: std::sync::mpsc::Sender<Record>) -> Self {
        Self {
            sender: Mutex::new(sender),
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let message = format!("{} - {}", record.level(), record.args());
            if let Err(_e) = self.sender.lock().unwrap().send(Record { message }) {
                // TODO: this is infinite recursion: warn!("{}", e);
            }
        }
    }
    fn flush(&self) {}
}

struct Channel {
    sender: Option<std::sync::mpsc::Sender<Record>>,
    receiver: Option<std::sync::mpsc::Receiver<Record>>,
}

static CHANNEL: once_cell::sync::Lazy<Mutex<Channel>> = once_cell::sync::Lazy::new(|| {
    let (sender, receiver) = std::sync::mpsc::channel();
    Mutex::new(Channel {
        sender: Some(sender),
        receiver: Some(receiver),
    })
});

fn logger() -> impl log::Log {
    Logger::new(CHANNEL.lock().unwrap().sender.take().unwrap())
}

pub struct Console {
    receiver: std::sync::mpsc::Receiver<Record>,
    records: std::collections::VecDeque<Record>,
}

impl Console {
    const MAX_RECORDS: usize = 10;
    pub fn new() -> Self {
        batbox_logger::add_logger(Box::new(logger()));
        let receiver = CHANNEL.lock().unwrap().receiver.take().unwrap();
        log::debug!("Debug overlay initialized");
        Self {
            receiver,
            records: std::collections::VecDeque::new(),
        }
    }

    pub fn update(&mut self, _delta_time: f64) {
        while let Ok(record) = self.receiver.try_recv() {
            self.records.push_back(record);
        }
        while self.records.len() > Self::MAX_RECORDS {
            self.records.pop_front();
        }
    }

    pub fn draw(&mut self, _framebuffer: &mut ugli::Framebuffer) {}

    #[allow(dead_code)]
    pub fn ui<'a>(&'a mut self, cx: &'a ui::Controller) -> impl ui::Widget + 'a {
        use ui::*;
        column(
            self.records
                .iter()
                .map(move |record| {
                    Box::new(
                        Text::new(&record.message, &cx.theme().font, 16.0, Rgba::WHITE)
                            .align(vec2(0.0, 0.5)),
                    ) as Box<_>
                })
                .collect(),
        )
        .align(vec2(0.0, 1.0))
    }
}
