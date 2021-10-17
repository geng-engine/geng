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
        self.components
            .remove(&TypeId::of::<T>())
            .map(|storage| storage.into_inner())
    }
    pub fn query<Q: Query>(&self) -> Option<EntityQuery<Q>> {
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
    pub fn borrow<T: Component>(&self) -> Option<storage::entity::Borrow<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|storage| storage.borrow())
    }
    pub fn borrow_mut<T: Component>(&self) -> Option<storage::entity::BorrowMut<T>> {
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
    pub fn query<Q: Query>(self) -> Option<EntityQuery<'a, Q>> {
        let query = Q::Fetch::default();
        let filter = self.filter.fetch();
        let filtered = {
            if let Some(mut borrows) = filter.borrow_direct(self.entity) {
                F::get(&filter, &mut borrows)
            } else {
                false
            }
        };
        if filtered {
            if let Some(borrows) = query.borrow_direct(self.entity) {
                return Some(EntityQuery {
                    query: &query,
                    borrows,
                });
            }
        }
        None
    }
}

pub struct EntityQuery<'a, Q: Query> {
    query: &'a Q::Fetch,
    borrows: <Q::Fetch as Fetch<'a>>::DirectBorrows,
}

impl<'a, Q: Query> EntityQuery<'a, Q> {
    pub fn get(&'a mut self) -> QueryOutput<'a, Q> {
        self.query.get(&mut self.borrows)
    }
}
