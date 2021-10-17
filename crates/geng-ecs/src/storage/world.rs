use super::*;

pub struct Storage {
    data: RefCell<HashMap<Id, Box<dyn Any>>>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
        }
    }
    pub fn insert_any(&mut self, id: Id, data: Box<dyn Any>) {
        self.data.get_mut().insert(id, data);
    }
    pub fn remove_any(&mut self, id: Id) -> Option<Box<dyn Any>> {
        self.data.get_mut().remove(&id)
    }
    pub fn borrow<T: Component>(&self) -> Borrow<T> {
        Borrow(self.data.borrow(), PhantomData)
    }
    pub fn borrow_mut<T: Component>(&self) -> BorrowMut<T> {
        BorrowMut(self.data.borrow_mut(), PhantomData)
    }
}

pub struct Borrow<'a, T: Component>(
    std::cell::Ref<'a, HashMap<Id, Box<dyn Any>>>,
    PhantomData<&'a T>,
);

impl<'a, T: Component> Borrow<'a, T> {
    pub fn get(&self, id: Id) -> Option<&T> {
        self.0.get(&id).map(|data| data.downcast_ref().unwrap())
    }
}

pub struct BorrowMut<'a, T: Component>(
    std::cell::RefMut<'a, HashMap<Id, Box<dyn Any>>>,
    PhantomData<&'a mut T>,
);

impl<'a, T: Component> BorrowMut<'a, T> {
    pub fn get(&self, id: Id) -> Option<&T> {
        self.0.get(&id).map(|data| data.downcast_ref().unwrap())
    }
    pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
        self.0.get_mut(&id).map(|data| data.downcast_mut().unwrap())
    }
}
