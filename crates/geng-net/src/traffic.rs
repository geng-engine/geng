use super::*;

#[derive(Clone, Debug, Default)]
pub struct Traffic {
    pub inbound: usize,
    pub outbound: usize,
}

impl Traffic {
    pub fn new() -> Self {
        Self {
            inbound: 0,
            outbound: 0,
        }
    }
}

pub struct TrafficWatcher {
    timer: Timer,
    start_value: Traffic,
    last_delta: Traffic,
}

impl std::fmt::Display for TrafficWatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "in: {}, out: {}",
            self.last_delta.inbound, self.last_delta.outbound
        )
    }
}

impl TrafficWatcher {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            timer: Timer::new(),
            start_value: Traffic::new(),
            last_delta: Traffic::new(),
        }
    }
    pub fn update(&mut self, traffic: &Traffic) {
        if self.timer.elapsed().as_secs_f64() > 1.0 {
            self.timer = Timer::new();
            self.last_delta = Traffic {
                inbound: traffic.inbound - self.start_value.inbound,
                outbound: traffic.outbound - self.start_value.outbound,
            };
            self.start_value = traffic.clone();
        }
    }
}
