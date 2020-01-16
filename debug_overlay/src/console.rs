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

static CHANNEL: once_cell::sync::Lazy<
    Mutex<(
        Option<std::sync::mpsc::Sender<Record>>,
        Option<std::sync::mpsc::Receiver<Record>>,
    )>,
> = once_cell::sync::Lazy::new(|| {
    let (sender, receiver) = std::sync::mpsc::channel();
    Mutex::new((Some(sender), Some(receiver)))
});

pub fn logger() -> impl log::Log {
    Logger::new(CHANNEL.lock().unwrap().0.take().unwrap())
}

pub struct Console {
    geng: Rc<Geng>,
    receiver: std::sync::mpsc::Receiver<Record>,
    records: std::collections::VecDeque<Record>,
}

impl Console {
    const MAX_RECORDS: usize = 10;
    pub fn new(geng: &Rc<Geng>) -> Self {
        let receiver = CHANNEL.lock().unwrap().1.take().unwrap();
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
                    Box::new(text(&record.message, font, 16.0, Color::WHITE).align(vec2(0.0, 0.5)))
                        as Box<_>
                })
                .collect(),
        )
        .align(vec2(0.0, 1.0))
    }
}
