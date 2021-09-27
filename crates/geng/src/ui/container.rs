pub trait Container {
    type Leaf;
    fn leaf(&self) -> &Self::Leaf;
}

impl<T: Container> Container for &mut T {
    type Leaf = T::Leaf;
    fn leaf(&self) -> &Self::Leaf {
        <T as Container>::leaf(self)
    }
}
