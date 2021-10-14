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
    pub fn add(&mut self, entity: Entity) {
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
    pub fn query<Q: Query>(&mut self) -> WorldQuery<Q> {
        self.filter(()).query()
    }
    pub fn filter<F: Filter>(&mut self, filter: F) -> FilteredWorld<F> {
        FilteredWorld {
            world: self,
            filter,
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
    world: &'a mut World,
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
                phantom_data: PhantomData,
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
    phantom_data: PhantomData<Q>,
}

impl<'a, Q: Query, F: Filter> WorldQuery<'a, Q, F> {
    pub fn iter<'q>(&'q self) -> WorldQueryIter<'a, 'q, Q, F> {
        WorldQueryIter {
            inner: self,
            id_iter: self.world.ids.iter(),
        }
    }
}

pub struct WorldQueryIter<'a, 'q, Q: Query, F: Filter> {
    inner: &'q WorldQuery<'a, Q, F>,
    id_iter: std::collections::hash_set::Iter<'a, Id>,
}

impl<'a, 'q, Q: Query, F: Filter> Iterator for WorldQueryIter<'a, 'q, Q, F> {
    type Item = QueryOutput<'a, Q>;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while let Some(&id) = self.id_iter.next() {
                let (querry_borrows, filter_borrows) = self.inner.borrows.as_ref()?;
                if !F::get_world(&self.inner.filter, filter_borrows, id) {
                    continue;
                }
                if let Some(item) = self.inner.query.get_world(querry_borrows, id) {
                    return Some(item);
                }
            }
            None
        }
    }
}
