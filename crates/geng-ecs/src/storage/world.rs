use super::*;

pub struct Storage {
    data: HashMap<Id, UnsafeCell<Box<dyn Any>>>,
    borrows: Cell<usize>,
    borrowed_mutably: Cell<bool>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
            borrows: Cell::new(0),
            borrowed_mutably: Cell::new(false),
        }
    }
    pub fn insert_any(&mut self, id: Id, data: Box<dyn Any>) {
        self.data.insert(id, UnsafeCell::new(data));
    }
    pub fn remove_any(&mut self, id: Id) -> Option<Box<dyn Any>> {
        self.data.remove(&id).map(|value| value.into_inner())
    }
    pub unsafe fn borrow<T: Component>(&self) -> Borrow<'_, T> {
        if self.borrowed_mutably.get() {
            panic!("Failed to borrow, already mutably borrowed");
        }
        self.borrows.set(self.borrows.get() + 1);
        Borrow(self, PhantomData)
    }
    pub unsafe fn borrow_mut<T: Component>(&self) -> BorrowMut<'_, T> {
        if self.borrows.get() != 0 {
            panic!("Failed to mutably borrow, already borrowed");
        }
        if self.borrowed_mutably.get() {
            panic!("Failed to mutably borrow, already mutably borrowed");
        }
        self.borrowed_mutably.set(true);
        BorrowMut(self, PhantomData)
    }
}

pub struct Borrow<'a, T: Component>(&'a Storage, PhantomData<&'a T>);

impl<'a, T: Component> Borrow<'a, T> {
    pub unsafe fn get(&self, id: Id) -> Option<&'a T> {
        self.0
            .data
            .get(&id)
            .map(|data| (*data.get()).downcast_ref().unwrap())
    }
}

impl<'a, T: Component> Drop for Borrow<'a, T> {
    fn drop(&mut self) {
        self.0.borrows.set(self.0.borrows.get() - 1);
    }
}

pub struct BorrowMut<'a, T: Component>(&'a Storage, PhantomData<&'a mut T>);

impl<'a, T: Component> BorrowMut<'a, T> {
    pub unsafe fn get(&self, id: Id) -> Option<&'a mut T> {
        self.0
            .data
            .get(&id)
            .map(|data| (*data.get()).downcast_mut().unwrap())
    }
}

impl<'a, T: Component> Drop for BorrowMut<'a, T> {
    fn drop(&mut self) {
        self.0.borrowed_mutably.set(false);
    }
}
