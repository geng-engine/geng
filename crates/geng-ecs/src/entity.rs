use super::*;

pub struct Entity {
    pub(crate) components: HashMap<TypeId, single_component_storage::Storage>,
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
    pub fn query<Q: Query>(&mut self) -> EntityQuery<Q> {
        self.query_filtered::<Q, ()>()
    }
    pub fn query_filtered<Q: Query, F: Filter>(&mut self) -> EntityQuery<Q> {
        unsafe {
            let filtered = {
                let borrows = F::borrow_direct(self);
                borrows.map_or(false, |borrows| <F as Filter>::get(&borrows))
            };
            if filtered {
                let borrows = Q::Fetch::borrow_direct(self);
                let item = borrows.as_ref().map(|borrows| Q::Fetch::get(borrows));
                EntityQuery { borrows, item }
            } else {
                EntityQuery {
                    borrows: None,
                    item: None,
                }
            }
        }
    }
    pub fn filter<F: Filter>(&mut self) -> bool {
        self.query_filtered::<(), F>().is_some()
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

pub struct EntityQuery<'a, Q: Query> {
    #[allow(dead_code)]
    borrows: Option<<Q::Fetch as Fetch<'a>>::DirectBorrows>, // This is here for the Drop impl
    item: Option<QueryOutput<'a, Q>>,
}

impl<'a, Q: Query> Debug for EntityQuery<'a, Q>
where
    QueryOutput<'a, Q>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.item)
    }
}

impl<'a, Q: Query> Deref for EntityQuery<'a, Q> {
    type Target = Option<QueryOutput<'a, Q>>;
    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl<'a, Q: Query> DerefMut for EntityQuery<'a, Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}
