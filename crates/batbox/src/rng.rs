//! Random number generation addons

use super::*;

#[doc(no_inline)]
pub use ::rand::{
    self,
    rngs::StdRng,
    seq::{IteratorRandom, SliceRandom},
    Rng, RngCore, SeedableRng,
};
// Note for web support: https://github.com/rust-random/rand#wasm-support
#[doc(no_inline)]
pub use ::rand::{rngs::ThreadRng, thread_rng};

/// Extends [Rng] with more methods
pub trait RngExt: Rng {
    /// Generate a uniformly distributed random point inside a circle
    fn gen_circle<T: Float>(&mut self, center: vec2<T>, radius: T) -> vec2<T> {
        let r = self.gen_range(0.0..=1.0).sqrt();
        let a = self.gen_range(0.0..=2.0 * f32::PI);
        let (sin, cos) = a.sin_cos();
        vec2(r * sin, r * cos).map(T::from_f32) * radius + center
    }
}

impl<T: Rng + ?Sized> RngExt for T {}
