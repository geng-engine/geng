use super::*;

pub fn default<T: Default>() -> T {
    T::default()
}

pub fn min_max<T: Ord>(a: T, b: T) -> (T, T) {
    if a.cmp(&b) == std::cmp::Ordering::Less {
        (a, b)
    } else {
        (b, a)
    }
}

impl<T: PartialOrd> PartialOrdExt for T {}

pub fn partial_min<T: PartialOrd>(a: T, b: T) -> T {
    a.partial_min(b)
}

pub fn partial_max<T: PartialOrd>(a: T, b: T) -> T {
    a.partial_max(b)
}

pub fn partial_min_max<T: PartialOrd>(a: T, b: T) -> (T, T) {
    a.partial_min_max(b)
}

pub trait PartialOrdExt: PartialOrd {
    fn partial_min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        self.partial_min_max(other).0
    }

    fn partial_max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        self.partial_min_max(other).1
    }

    fn partial_min_max(self, other: Self) -> (Self, Self)
    where
        Self: Sized,
    {
        if self.partial_cmp(&other).unwrap() == std::cmp::Ordering::Less {
            (self, other)
        } else {
            (other, self)
        }
    }

    /// Clamps a value in range.
    /// # Panics
    /// Panics if range is exclusive.
    /// # Examples
    /// ```
    /// # use batbox::prelude::*;
    /// assert_eq!(2.0.clamp_range(0.0..=1.0), 1.0);
    /// assert_eq!(2.0.clamp_range(3.0..), 3.0);
    /// assert_eq!(2.0.clamp_range(..=0.0), 0.0);
    /// ```
    fn clamp_range(mut self, range: impl RangeBounds<Self>) -> Self
    where
        Self: Clone,
    {
        match range.start_bound().cloned() {
            Bound::Included(start) => self = self.partial_max(start),
            Bound::Excluded(_) => panic!("Clamping with an exclusive range is undefined"),
            Bound::Unbounded => (),
        }
        match range.end_bound().cloned() {
            Bound::Included(end) => self = self.partial_min(end),
            Bound::Excluded(_) => panic!("Clamping with an exclusive range is undefined"),
            Bound::Unbounded => (),
        }
        self
    }

    fn clamp_abs(self, max: Self) -> Self
    where
        Self: Neg<Output = Self> + Copy,
    {
        self.clamp_range(-max..=max)
    }
}

pub fn index_range<R>(len: usize, range: R) -> Range<usize>
where
    R: RangeBounds<usize>,
{
    Range {
        start: match range.start_bound() {
            Bound::Included(&i) => i,
            Bound::Excluded(&i) => i + 1,
            Bound::Unbounded => 0,
        },
        end: match range.end_bound() {
            Bound::Included(&i) => i - 1,
            Bound::Excluded(&i) => i,
            Bound::Unbounded => len,
        },
    }
}

pub fn global_threadpool() -> &'static ThreadPool {
    static mut INSTANCE: Option<ThreadPool> = None;
    static mut INIT: std::sync::Once = std::sync::Once::new();
    unsafe {
        INIT.call_once(|| {
            mem::forget(mem::replace(&mut INSTANCE, Some(default())));
        });
        INSTANCE.as_ref().unwrap()
    }
}

pub fn static_path() -> std::path::PathBuf {
    if let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        let mut path = std::path::PathBuf::from(dir);
        let current_exe = std::env::current_exe().unwrap();
        if let Some(binary_type) = current_exe.parent() {
            if binary_type.file_name().unwrap() == "examples" {
                path = path.join("examples").join(current_exe.file_stem().unwrap());
            }
        }
        let path = path.join("static");
        if path.is_dir() {
            return path;
        }
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = std::env::current_exe().unwrap().parent() {
                return path.to_owned();
            }
        }
    }
    if cfg!(target_arch = "wasm32") {
        std::path::PathBuf::from(".")
    } else {
        std::env::current_dir().unwrap()
    }
}
