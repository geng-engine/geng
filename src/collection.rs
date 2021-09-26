use super::*;
use std::hash::Hash;

pub trait HasId {
    type Id: Debug
        + Clone
        + Hash
        + Eq
        + Serialize
        + for<'de> Deserialize<'de>
        + 'static
        + Send
        + Sync
        + Unpin;
    fn id(&self) -> &Self::Id;
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Collection<T: HasId> {
    by_id: HashMap<T::Id, T>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollectionDelta<T: HasId + Diff> {
    #[serde(bound = "")]
    pub new_entities: Vec<T>,
    pub deleted_entities: Vec<T::Id>,
    #[serde(bound = "")]
    pub mutated_entities: Vec<(T::Id, T::Delta)>,
}

impl<T: HasId + Clone + Diff> Diff for Collection<T> {
    type Delta = CollectionDelta<T>;
    fn diff(&self, to: &Self) -> Self::Delta {
        let mut new_entities = Vec::new();
        let mut deleted_entities = Vec::new();
        let mut mutated_entities = Vec::new();
        for entity in to {
            match self.get(entity.id()) {
                Some(old_entity) => {
                    if entity != old_entity {
                        mutated_entities.push((entity.id().clone(), old_entity.diff(entity)));
                    }
                }
                None => new_entities.push(entity.clone()),
            }
        }
        for entity in self {
            if to.get(entity.id()).is_none() {
                deleted_entities.push(entity.id().clone());
            }
        }
        CollectionDelta {
            new_entities,
            deleted_entities,
            mutated_entities,
        }
    }
    fn update(&mut self, delta: &Self::Delta) {
        for id in &delta.deleted_entities {
            self.remove(id);
        }
        for (id, delta) in &delta.mutated_entities {
            self.get_mut(id)
                .expect("Delta was not built correctly")
                .update(delta);
        }
        for entity in &delta.new_entities {
            self.insert(entity.clone());
        }
    }
}

impl<T: HasId> Collection<T> {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
        }
    }
    pub fn insert(&mut self, obj: T) {
        self.by_id.insert(obj.id().clone(), obj);
    }
    pub fn get(&self, id: &T::Id) -> Option<&T> {
        self.by_id.get(&id)
    }
    pub fn get_mut(&mut self, id: &T::Id) -> Option<&mut T> {
        self.by_id.get_mut(&id)
    }
    pub fn remove(&mut self, id: &T::Id) -> Option<T> {
        self.by_id.remove(&id)
    }
    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        self.by_id.retain(move |_, obj| f(obj));
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.into_iter()
    }
    pub fn ids(&self) -> impl Iterator<Item = &T::Id> + '_ {
        self.by_id.keys()
    }
    pub fn len(&self) -> usize {
        self.by_id.len()
    }
}

impl<T: HasId> Extend<T> for Collection<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, entities: I) {
        self.by_id
            .extend(entities.into_iter().map(|obj| (obj.id().clone(), obj)));
    }
}

impl<T: HasId> std::iter::FromIterator<T> for Collection<T> {
    fn from_iter<I: IntoIterator<Item = T>>(entities: I) -> Self {
        let mut result = Self::new();
        result.extend(entities);
        result
    }
}

// TODO: allow not 'static somehow
impl<T: HasId + 'static> IntoIterator for Collection<T> {
    type Item = T;
    type IntoIter = Box<dyn Iterator<Item = T>>;
    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.by_id.into_iter().map(|(_, obj)| obj))
    }
}

impl<'a, T: HasId> IntoIterator for &'a Collection<T> {
    type Item = &'a T;
    type IntoIter = Box<dyn Iterator<Item = &'a T> + 'a>;
    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.by_id.values())
    }
}

impl<'a, T: HasId> IntoIterator for &'a mut Collection<T> {
    type Item = &'a mut T;
    type IntoIter = Box<dyn Iterator<Item = &'a mut T> + 'a>;
    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.by_id.values_mut())
    }
}
