//! Time related things
//!
//! [std::time] is not working on web so use this instead

#[allow(unused_imports)]
use super::*;

pub mod prelude {
    //! Items intended to always be available. Reexported from [crate::prelude]

    #[doc(no_inline)]
    pub use super::*;
}

/// A measurement of a monotonically nondecreasing clock.
///
/// Alternative of [std::time::Instant]
#[derive(Copy, Clone)]
pub struct Instant {
    #[cfg(target_arch = "wasm32")]
    value: f64,
    #[cfg(not(target_arch = "wasm32"))]
    inner: std::time::Instant,
}

impl Instant {
    /// Returns an instant corresponding to "now".
    pub fn now() -> Self {
        Self {
            #[cfg(target_arch = "wasm32")]
            value: {
                thread_local! {
                    static PERFORMANCE: web_sys::Performance = web_sys::window()
                        .expect("no window")
                        .performance()
                        .expect("no performance");
                }
                PERFORMANCE.with(|performance| performance.now() / 1000.0)
            },
            #[cfg(not(target_arch = "wasm32"))]
            inner: std::time::Instant::now(),
        }
    }

    /// Returns the amount of time elapsed from another instant to this one
    pub fn duration_since(&self, earlier: Self) -> Duration {
        #[cfg(target_arch = "wasm32")]
        return Duration::from_secs_f64(self.value - earlier.value);
        #[cfg(not(target_arch = "wasm32"))]
        return Duration::from_secs_f64(self.inner.duration_since(earlier.inner).as_secs_f64());
    }

    /// Returns the amount of time elapsed since this instant
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }
}

/// Represents a span of time.
///
/// Alternative of [std::time::Duration]
#[derive(Copy, Clone)]
pub struct Duration {
    secs: f64,
}

impl Add for Duration {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            secs: self.secs + rhs.secs,
        }
    }
}

impl Debug for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        std::time::Duration::from(*self).fmt(f)
    }
}

impl Duration {
    /// Creates a new Duration from the specified number of seconds represented as f64.
    pub fn from_secs_f64(secs: f64) -> Self {
        Self { secs }
    }

    /// Returns the number of seconds contained by this Duration as f64
    pub fn as_secs_f64(&self) -> f64 {
        self.secs
    }
}

impl From<Duration> for std::time::Duration {
    fn from(value: Duration) -> Self {
        std::time::Duration::from_secs_f64(value.as_secs_f64())
    }
}

impl From<std::time::Duration> for Duration {
    fn from(value: std::time::Duration) -> Self {
        Duration::from_secs_f64(value.as_secs_f64())
    }
}

/// Timer can be used to track time since some instant
pub struct Timer {
    start: Instant,
}

impl Timer {
    #[allow(clippy::new_without_default)]
    /// Constructs a new timer.
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Get duration elapsed since last reset.
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Reset, and get time elapsed since last reset.
    pub fn tick(&mut self) -> Duration {
        let now = Instant::now();
        let duration = now.duration_since(self.start);
        self.start = now;
        duration
    }
}

#[test]
fn test() {
    let mut timer = Timer::new();
    timer.elapsed();
    for _ in 0..100 {
        timer.tick();
    }
}
