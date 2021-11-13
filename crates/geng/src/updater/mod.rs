mod updatable;

pub use updatable::*;

/// Can be used to call functions with a fixed period (i.e. 5 times a second).
pub struct FixedUpdater<T: Updatable> {
    pub fixed_delta_time: f32,
    pub contents: T,
    current_time: f32,
}

impl<T: Updatable> FixedUpdater<T> {
    /// Initialize a new [FixedUpdater] with given `fixed_delta_time`
    /// and delayed first update (specified with `delay` argument).
    pub fn new(fixed_delta_time: f32, delay: f32, contents: T) -> Self {
        assert!(fixed_delta_time >= 0.0, "Attempted to create a FixedUpdater with negative fixed_delta_time ({}), which is nonsense.", fixed_delta_time);
        assert!(
            delay >= 0.0,
            "Attempted to create a FixedUpdater with negative delay ({}), which is nonsense.",
            delay
        );
        Self {
            fixed_delta_time,
            current_time: fixed_delta_time - delay,
            contents,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.current_time += delta_time;
        while self.current_time >= self.fixed_delta_time {
            self.current_time -= self.fixed_delta_time;
            self.contents.update(self.fixed_delta_time);
        }
    }
}
