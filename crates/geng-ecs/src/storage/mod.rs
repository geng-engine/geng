use super::*;

pub mod entity;
pub mod world;

pub use entity::Storage as Entity;
pub use world::Storage as World;

// GAT waiting room
// https://github.com/rust-lang/rust/issues/44265

// pub  trait Storage {
//     type Ref<'a, T: Component>: Ref<'a, T>;
//     fn borrow<'a, T: Component>(&'a self) -> Self::Ref<'a, T>;

//     type RefMut<'a, T: Component>: Ref<'a, T>;
//     fn borrow_mut<'a, T: Component>(&'a self) -> Self::RefMut<'a, T>;
// }

// pub  trait Ref<'a, T: Component> {
//     type Key;
//     fn get(&self, key: Self::Key) -> &'a T;
// }

// pub  trait RefMut<'a, T: Component> {
//     type Key;
//     fn get(&self, key: Self::Key) -> &'a mut T;
// }
