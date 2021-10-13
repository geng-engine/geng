use super::*;

pub struct Entity {
    components: HashMap<TypeId, single_component_storage::Storage>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
    pub fn add<T: Component>(&mut self, component: T) {
        self.components.insert(
            TypeId::of::<T>(),
            single_component_storage::Storage::new(component),
        );
    }
    pub fn has<T: Component>(&self) -> bool {
        self.components.contains_key(&TypeId::of::<T>())
    }
    pub fn remove<T: Component>(&mut self) -> Option<T> {
        unsafe {
            self.components
                .remove(&TypeId::of::<T>())
                .map(|storage| storage.into_inner())
        }
    }
    pub fn query<'a, Q: Query<'a>>(&'a mut self) -> EntityQuery<'a, Q> {
        unsafe {
            let borrows = Q::borrow_direct(self);
            let item = borrows.as_ref().map(|borrows| Q::get(borrows));
            EntityQuery { borrows, item }
        }
    }
    pub unsafe fn borrow<T: Component>(&self) -> Option<single_component_storage::Borrow<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|storage| storage.borrow())
    }
    pub unsafe fn borrow_mut<T: Component>(
        &self,
    ) -> Option<single_component_storage::BorrowMut<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|storage| storage.borrow_mut())
    }
}

pub struct EntityQuery<'a, Q: Query<'a>> {
    #[allow(dead_code)]
    borrows: Option<Q::DirectBorrows>, // This is here for the Drop impl
    item: Option<Q::Output>,
}

impl<'a, Q: Query<'a>> Debug for EntityQuery<'a, Q>
where
    Q::Output: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.item)
    }
}

impl<'a, Q: Query<'a>> Deref for EntityQuery<'a, Q> {
    type Target = Option<Q::Output>;
    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<'a, Q: Query<'a>> DerefMut for EntityQuery<'a, Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}
