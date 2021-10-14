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
    pub fn into_inner_any(self) -> Box<dyn Any> {
        assert_eq!(self.borrows.get(), 0, "Component is still borrowed");
        assert!(
            !self.borrowed_mutably.get(),
            "Component is still mutably borrowed"
        );
        self.data.into_inner()
    }
    pub unsafe fn into_inner<T: Component>(self) -> T {
        *self.into_inner_any().downcast().unwrap()
    }
    pub unsafe fn borrow<T: Component>(&self) -> Borrow<T> {
        if self.borrowed_mutably.get() {
            panic!("Failed to borrow, already mutably borrowed");
        }
        self.borrows.set(self.borrows.get() + 1);
        Borrow(self, (*self.data.get()).downcast_ref().unwrap())
    }
    pub unsafe fn borrow_mut<T: Component>(&self) -> BorrowMut<T> {
        if self.borrows.get() != 0 {
            panic!("Failed to mutably borrow, already borrowed");
        }
        if self.borrowed_mutably.get() {
            panic!("Failed to mutably borrow, already mutably borrowed");
        }
        self.borrowed_mutably.set(true);
        BorrowMut(self, (*self.data.get()).downcast_mut().unwrap())
    }
}

pub struct Borrow<'a, T: Component>(&'a Storage, *const T);

impl<'a, T: Component> Borrow<'a, T> {
    pub unsafe fn get(&self) -> &'a T {
        &*self.1
    }
}

impl<'a, T: Component> Drop for Borrow<'a, T> {
    fn drop(&mut self) {
        self.0.borrows.set(self.0.borrows.get() - 1);
    }
}

pub struct BorrowMut<'a, T: Component>(&'a Storage, *mut T);

impl<'a, T: Component> BorrowMut<'a, T> {
    pub unsafe fn get(&self) -> &'a mut T {
        &mut *self.1
    }
}

impl<'a, T: Component> Drop for BorrowMut<'a, T> {
    fn drop(&mut self) {
        self.0.borrowed_mutably.set(false);
    }
}
