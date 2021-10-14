use super::*;

pub struct World {
    pub(crate) components: HashMap<TypeId, component_storage::Storage>,
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
                .or_insert(component_storage::Storage::new())
                .insert_any(id, value.into_inner_any());
        }
        self.ids.insert(id);
    }
    pub fn query<Q: Query>(&mut self) -> WorldQuery<Q> {
        self.query_filtered()
    }
    pub fn query_filtered<Q: Query, F: Filter>(&mut self) -> WorldQuery<Q, F> {
        unsafe fn borrow<'a, Q: Query, F: Filter>(world: &'a World) -> Option<Borrows<'a, Q, F>> {
            Some((
                Q::Fetch::borrow_world(world)?,
                F::Fetch::borrow_world(world)?,
            ))
        }
        unsafe {
            WorldQuery {
                borrows: borrow::<Q, F>(self),
                world: self,
                phantom_data: PhantomData,
            }
        }
    }
    pub unsafe fn borrow<T: Component>(&self) -> Option<component_storage::Borrow<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|storage| storage.borrow())
    }
    pub unsafe fn borrow_mut<T: Component>(&self) -> Option<component_storage::BorrowMut<T>> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|storage| storage.borrow_mut())
    }
}

type Borrows<'a, Q, F> = (
    <<Q as Query>::Fetch as Fetch<'a>>::WorldBorrows,
    <<F as Filter>::Fetch as Fetch<'a>>::WorldBorrows,
);

pub struct WorldQuery<'a, Q: Query, F: Filter = ()> {
    #[allow(dead_code)]
    borrows: Option<Borrows<'a, Q, F>>, // This is here for the Drop impl
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
                if !<F as Filter>::get_world(filter_borrows, id) {
                    continue;
                }
                if let Some(item) = Q::Fetch::get_world(querry_borrows, id) {
                    return Some(item);
                }
            }
            None
        }
    }
}
