/// Can be used to call functions with a fixed period (i.e. 5 times a second).
pub struct FixedUpdater {
    pub fixed_delta_time: f32,
    current_time: f32,
}

impl FixedUpdater {
    /// Initialize a new [FixedUpdater] with given `fixed_delta_time`
    /// and delayed first update (specified with `delay` argument).
    pub fn new(fixed_delta_time: f32, delay: f32) -> Self {
        assert!(fixed_delta_time >= 0.0, "Attempted to create a FixedUpdater with negative fixed_delta_time ({}), which is nonsense.", fixed_delta_time);
        assert!(
            delay >= 0.0,
            "Attempted to create a FixedUpdater with negative delay ({}), which is nonsense.",
            delay
        );
        Self {
            fixed_delta_time,
            current_time: fixed_delta_time - delay,
        }
    }

    pub fn update(&mut self, delta_time: f32) -> usize {
        self.current_time += delta_time;
        let mut updates = 0;
        while self.current_time >= self.fixed_delta_time {
            self.current_time -= self.fixed_delta_time;
            updates += 1;
        }
        updates
    }
}
