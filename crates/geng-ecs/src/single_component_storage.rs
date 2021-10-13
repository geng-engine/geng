use super::*;

pub struct Storage {
    data: UnsafeCell<Box<dyn Any>>,
    borrows: Cell<usize>,
    borrowed_mutably: Cell<bool>,
}

impl Storage {
    pub fn new<T: Component>(value: T) -> Self {
        Self {
            data: UnsafeCell::new(Box::new(value)),
            borrows: Cell::new(0),
            borrowed_mutably: Cell::new(false),
        }
    }
    pub unsafe fn into_inner<T: Component>(self) -> T {
        assert_eq!(self.borrows.get(), 0, "Component is still borrowed");
        assert!(
            !self.borrowed_mutably.get(),
            "Component is still mutably borrowed"
        );
        *self.data.into_inner().downcast().unwrap()
    }
    pub unsafe fn borrow(&self) -> Borrow {
        if self.borrowed_mutably.get() {
            panic!("Failed to borrow, already mutably borrowed");
        }
        self.borrows.set(self.borrows.get() + 1);
        Borrow(self)
    }
    pub unsafe fn get<T: Component>(&self) -> &T {
        (*self.data.get()).downcast_ref().unwrap()
    }
    unsafe fn release(&self) {
        self.borrows.set(self.borrows.get() - 1);
    }
    pub unsafe fn borrow_mut(&self) -> BorrowMut {
        if self.borrows.get() != 0 {
            panic!("Failed to mutably borrow, already borrowed");
        }
        if self.borrowed_mutably.get() {
            panic!("Failed to mutably borrow, already mutably borrowed");
        }
        self.borrowed_mutably.set(true);
        BorrowMut(self)
    }
    pub unsafe fn get_mut<T: Component>(&self) -> &mut T {
        (*self.data.get()).downcast_mut().unwrap()
    }
    unsafe fn release_mut(&self) {
        self.borrowed_mutably.set(false);
    }
}

pub struct Borrow<'a>(&'a Storage);

impl<'a> Drop for Borrow<'a> {
    fn drop(&mut self) {
        unsafe {
            self.0.release();
        }
    }
}

pub struct BorrowMut<'a>(&'a Storage);

impl<'a> Drop for BorrowMut<'a> {
    fn drop(&mut self) {
        unsafe {
            self.0.release_mut();
        }
    }
}
