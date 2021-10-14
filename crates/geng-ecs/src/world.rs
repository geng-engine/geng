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
    pub fn query<'a, Q: Query<'a>>(&'a mut self) -> WorldQuery<'a, Q, ()> {
        self.query_filtered::<Q, ()>()
    }
    pub fn query_filtered<'a, Q: Query<'a>, F: Filter<'a>>(&'a mut self) -> WorldQuery<'a, Q, F> {
        unsafe fn borrow<'a, Q: Query<'a>, F: Filter<'a>>(
            world: &'a World,
        ) -> Option<(Q::WorldBorrows, F::WorldBorrows)> {
            Some((Q::borrow_world(world)?, F::borrow_world(world)?))
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

pub struct WorldQuery<'a, Q: Query<'a>, F: Filter<'a>> {
    #[allow(dead_code)]
    borrows: Option<(Q::WorldBorrows, F::WorldBorrows)>, // This is here for the Drop impl
    world: &'a World,
    phantom_data: PhantomData<Q>,
}

impl<'a, Q: Query<'a>, F: Filter<'a>> WorldQuery<'a, Q, F> {
    pub fn iter<'q>(&'q self) -> WorldQueryIter<'a, 'q, Q, F> {
        WorldQueryIter {
            inner: self,
            id_iter: self.world.ids.iter(),
        }
    }
}

pub struct WorldQueryIter<'a, 'q, Q: Query<'a>, F: Filter<'a>> {
    inner: &'q WorldQuery<'a, Q, F>,
    id_iter: std::collections::hash_set::Iter<'a, Id>,
}

impl<'a, 'q, Q: Query<'a>, F: Filter<'a>> Iterator for WorldQueryIter<'a, 'q, Q, F> {
    type Item = Q;
    fn next(&mut self) -> Option<Q> {
        unsafe {
            while let Some(&id) = self.id_iter.next() {
                let (querry_borrows, filter_borrows) = self.inner.borrows.as_ref()?;
                if !F::get_world(filter_borrows, id) {
                    continue;
                }
                if let Some(item) = Q::get_world(querry_borrows, id) {
                    return Some(item);
                }
            }
            None
        }
    }
}
