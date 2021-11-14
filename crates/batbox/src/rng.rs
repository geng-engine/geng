use super::*;

#[doc(no_inline)]
pub use rand::distributions::Distribution;
#[doc(no_inline)]
pub use rand::seq::{IteratorRandom as _, SliceRandom as _};
#[doc(no_inline)]
pub use rand::{self, rngs::StdRng, Rng, RngCore, SeedableRng};

pub mod distributions {
    use super::*;

    #[doc(no_inline)]
    pub use rand::distributions::*;

    pub struct UnitCircleInside;

    impl Distribution<Vec2<f32>> for UnitCircleInside {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec2<f32> {
            let r = rng.gen_range(0.0..=1.0).sqrt();
            let a = rng.gen_range(0.0..=2.0 * std::f32::consts::PI);
            vec2(r * a.sin(), r * a.cos())
        }
    }
}

pub fn global_rng() -> impl Rng {
    #[cfg(target_arch = "wasm32")]
    {
        static GLOBAL_RNG: once_cell::sync::Lazy<Mutex<StdRng>> =
            once_cell::sync::Lazy::new(|| {
                fn gen_byte() -> u8 {
                    (js_sys::Math::random() * 256.0).clamp(0.0, 255.0) as u8
                }
                let mut seed: [mem::MaybeUninit<u8>; 32] =
                    unsafe { mem::MaybeUninit::uninit().assume_init() };
                for elem in &mut seed {
                    unsafe {
                        std::ptr::write(elem.as_mut_ptr(), gen_byte());
                    }
                }
                Mutex::new(rand::SeedableRng::from_seed(unsafe {
                    mem::transmute(seed)
                }))
            });

        struct GlobalRng;

        impl RngCore for GlobalRng {
            fn next_u32(&mut self) -> u32 {
                GLOBAL_RNG.lock().unwrap().next_u32()
            }
            fn next_u64(&mut self) -> u64 {
                GLOBAL_RNG.lock().unwrap().next_u64()
            }
            fn fill_bytes(&mut self, dest: &mut [u8]) {
                GLOBAL_RNG.lock().unwrap().fill_bytes(dest);
            }
            fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
                GLOBAL_RNG.lock().unwrap().try_fill_bytes(dest)
            }
        }

        GlobalRng
    }
    #[cfg(not(target_arch = "wasm32"))]
    rand::thread_rng()
}

#[test]
fn test_random() {
    macro_rules! test_types {
        ($($t:ty,)*) => {
            $(eprintln!("random {:?} = {:?}", stringify!($t), global_rng().gen::<$t>());)*
        };
    }
    test_types!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, char, f32, f64,);
}
