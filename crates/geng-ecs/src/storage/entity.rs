use super::*;

pub struct Storage {
    data: RefCell<Box<dyn Any>>,
}

impl Storage {
    pub fn new<T: Component>(value: T) -> Self {
        Self::new_any(Box::new(value))
    }
    pub fn new_any(value: Box<dyn Any>) -> Self {
        Self {
            data: RefCell::new(value),
        }
    }
    pub fn into_inner_any(self) -> Box<dyn Any> {
        self.data.into_inner()
    }
    pub fn into_inner<T: Component>(self) -> T {
        *self.into_inner_any().downcast().unwrap()
    }
    pub fn borrow<T: Component>(&self) -> Borrow<T> {
        std::cell::Ref::map(self.data.borrow(), |data| data.downcast_ref().unwrap())
    }
    pub fn borrow_mut<T: Component>(&self) -> BorrowMut<T> {
        std::cell::RefMut::map(self.data.borrow_mut(), |data| data.downcast_mut().unwrap())
    }
}

pub type Borrow<'a, T> = std::cell::Ref<'a, T>;
pub type BorrowMut<'a, T> = std::cell::RefMut<'a, T>;
