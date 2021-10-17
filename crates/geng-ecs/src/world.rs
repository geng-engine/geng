use super::*;

pub struct World {
    pub(crate) components: HashMap<TypeId, storage::World>,
    ids: HashSet<Id>,
    next_id: u32,
}

impl World {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            ids: HashSet::new(),
            next_id: 0,
        }
    }
    pub fn spawn(&mut self, entity: Entity) {
        let id = Id(self.next_id);
        self.next_id += 1;
        for (type_id, value) in entity.components {
            self.components
                .entry(type_id)
                .or_insert(storage::World::new())
                .insert_any(id, value.into_inner_any());
        }
        self.ids.insert(id);
    }
    pub fn query<Q: Query>(&self) -> WorldQuery<Q> {
        self.filter(()).query()
    }
    pub fn filter<F: Filter>(&self, filter: F) -> FilteredWorld<F> {
        FilteredWorld {
            world: self,
            filter,
        }
    }
    pub fn remove<F: Filter>(&mut self, filter: F) -> WorldRemove<F> {
        let filter = filter.fetch();
        let iter = self.ids.clone().into_iter();
        WorldRemove {
            world: self,
            filter,
            id_iter: iter,
        }
    }
    pub unsafe fn borrow<T: Component>(&self) -> Option<storage::world::Borrow<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|storage| storage.borrow())
    }
    pub unsafe fn borrow_mut<T: Component>(&self) -> Option<storage::world::BorrowMut<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|storage| storage.borrow_mut())
    }
}

pub struct FilteredWorld<'a, T> {
    world: &'a World,
    filter: T,
}

impl<'a, F: Filter> FilteredWorld<'a, F> {
    pub fn query<Q: Query>(self) -> WorldQuery<'a, Q, F> {
        unsafe fn borrow<'a, Q: Query, F: Filter>(
            query: &Q::Fetch,
            filter: &F::Fetch,
            world: &'a World,
        ) -> Option<Borrows<'a, Q, F>> {
            Some((query.borrow_world(world)?, filter.borrow_world(world)?))
        }
        unsafe {
            let query = Q::Fetch::default();
            let filter = self.filter.fetch();
            WorldQuery {
                borrows: borrow::<Q, F>(&query, &filter, self.world),
                query,
                filter,
                world: self.world,
            }
        }
    }
    pub fn filter<F2: Filter>(self, filter: F2) -> FilteredWorld<'a, (F, F2)> {
        FilteredWorld {
            world: self.world,
            filter: (self.filter, filter),
        }
    }
}

type Borrows<'a, Q, F> = (
    <<Q as Query>::Fetch as Fetch<'a>>::WorldBorrows,
    <<F as Filter>::Fetch as Fetch<'a>>::WorldBorrows,
);

pub struct WorldQuery<'a, Q: Query, F: Filter = ()> {
    #[allow(dead_code)]
    borrows: Option<Borrows<'a, Q, F>>, // This is here for the Drop impl
    query: Q::Fetch,
    filter: F::Fetch,
    world: &'a World,
}

impl<'a, Q: Query, F: Filter> WorldQuery<'a, Q, F> {
    pub fn iter<'q>(&'q mut self) -> WorldQueryIter<'q, Q, F> {
        WorldQueryIter::<'q, Q, F> {
            borrows: unsafe { std::mem::transmute(self.borrows.as_ref()) }, // TODO: WTF
            query: &self.query,
            filter: &self.filter,
            id_iter: self.world.ids.iter(),
        }
    }
}

pub struct WorldQueryIter<'a, Q: Query, F: Filter> {
    borrows: Option<&'a Borrows<'a, Q, F>>,
    query: &'a Q::Fetch,
    filter: &'a F::Fetch,
    id_iter: std::collections::hash_set::Iter<'a, Id>,
}

impl<'a, Q: Query, F: Filter> Iterator for WorldQueryIter<'a, Q, F> {
    type Item = QueryOutput<'a, Q>;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while let Some(&id) = self.id_iter.next() {
                let (querry_borrows, filter_borrows) = self.borrows?;
                if !F::get_world(self.filter, filter_borrows, id) {
                    continue;
                }
                if let Some(item) = self.query.get_world(querry_borrows, id) {
                    return Some(item);
                }
            }
            None
        }
    }
}

pub struct WorldRemove<'a, F: Filter> {
    world: &'a mut World,
    filter: F::Fetch,
    id_iter: std::collections::hash_set::IntoIter<Id>,
}

impl<'a, F: Filter> Iterator for WorldRemove<'a, F> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        while let Some(id) = self.id_iter.next() {
            unsafe {
                if let Some(filter_borrows) = self.filter.borrow_world(self.world) {
                    if !F::get_world(&self.filter, &filter_borrows, id) {
                        continue;
                    }
                }
            }
            let mut entity = Entity::new();
            for (&type_id, storage) in &mut self.world.components {
                if let Some(value) = storage.remove_any(id) {
                    entity
                        .components
                        .insert(type_id, storage::Entity::new_any(value));
                }
            }
            self.world.ids.remove(&id);
            return Some(entity);
        }
        None
    }
}
