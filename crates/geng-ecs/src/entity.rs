use super::*;

pub struct Entity {
    pub(crate) components: HashMap<TypeId, storage::Entity>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }
    pub fn add<T: Component>(&mut self, component: T) {
        self.components
            .insert(TypeId::of::<T>(), storage::Entity::new(component));
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
    pub fn query<Q: Query>(&self) -> EntityQuery<Q> {
        self.filter(()).query()
    }
    pub fn filter<F: Filter>(&self, filter: F) -> FilteredEntity<F> {
        FilteredEntity {
            entity: self,
            filter,
        }
    }
    pub fn is<F: Filter>(&mut self, filter: F) -> bool {
        self.filter(filter).query::<()>().is_some()
    }
    pub unsafe fn borrow<T: Component>(&self) -> Option<storage::entity::Borrow<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|storage| storage.borrow())
    }
    pub unsafe fn borrow_mut<T: Component>(&self) -> Option<storage::entity::BorrowMut<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|storage| storage.borrow_mut())
    }
}

pub struct FilteredEntity<'a, T> {
    entity: &'a Entity,
    filter: T,
}

impl<'a, F: Filter> FilteredEntity<'a, F> {
    pub fn query<Q: Query>(self) -> EntityQuery<'a, Q> {
        unsafe {
            let query = Q::Fetch::default();
            let filter = self.filter.fetch();
            let filtered = {
                if let Some(borrows) = filter.borrow_direct(self.entity) {
                    F::get(&filter, &borrows)
                } else {
                    false
                }
            };
            if filtered {
                let borrows = query.borrow_direct(self.entity);
                let item = borrows.as_ref().map(|borrows| query.get(borrows));
                EntityQuery { borrows, item }
            } else {
                EntityQuery {
                    borrows: None,
                    item: None,
                }
            }
        }
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
