use super::*;

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
            if let Err(e) = self.sender.lock().unwrap().send(Record { message }) {
                warn!("{}", e);
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
    geng: Geng,
    receiver: std::sync::mpsc::Receiver<Record>,
    records: std::collections::VecDeque<Record>,
}

impl Console {
    const MAX_RECORDS: usize = 10;
    pub fn new(geng: &Geng) -> Self {
        logger::add_logger(Box::new(logger()));
        let receiver = CHANNEL.lock().unwrap().receiver.take().unwrap();
        info!("Debug overlay initialized");
        Self {
            geng: geng.clone(),
            receiver,
            records: std::collections::VecDeque::new(),
        }
    }
    pub fn before_draw(&mut self) {
        while let Ok(record) = self.receiver.try_recv() {
            self.records.push_back(record);
        }
        while self.records.len() > Self::MAX_RECORDS {
            self.records.pop_front();
        }
    }
    pub fn ui(&mut self) -> impl ui::Widget + '_ {
        use ui::*;
        let font = self.geng.default_font();
        column(
            self.records
                .iter()
                .map(move |record| {
                    Box::new(
                        Text::new(&record.message, font, 16.0, Color::WHITE).align(vec2(0.0, 0.5)),
                    ) as Box<_>
                })
                .collect(),
        )
        .align(vec2(0.0, 1.0))
    }
}
