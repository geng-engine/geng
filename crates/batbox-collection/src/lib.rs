//! A collection of identifiable entities

use batbox_diff::Diff;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub use batbox_collection_derive::*;

/// An identifiable entity
///
/// The contract here is to make sure entity's id stays the same for its lifetime
///
/// [Can be derived](::batbox_derive::HasId)
pub trait HasId {
    /// Type of the entity's id
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

    /// Get the entity's id
    fn id(&self) -> &Self::Id;
}

/// A collection of identifiable entities
///
/// This behaves kind of like a HashSet, but only entities' ids are used
/// Another difference is that it allows for getting mutable references
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Collection<T: HasId> {
    by_id: HashMap<T::Id, T>,
}

/// A difference between two collections
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollectionDelta<T: HasId + Diff> {
    /// Entities present in second, but not in first collection
    #[serde(bound = "")]
    pub new_entities: Vec<T>,
    /// Entities present in first, but not in second collection
    pub deleted_entities: Vec<T::Id>,
    /// List of deltas for mutated entities (present in both collections that are not equal)
    #[serde(bound = "")]
    pub mutated_entities: Vec<(T::Id, T::Delta)>,
}

impl<T: HasId + Clone + Diff> Diff for Collection<T> {
    type Delta = Option<CollectionDelta<T>>;
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
        if new_entities.is_empty() && deleted_entities.is_empty() && mutated_entities.is_empty() {
            None
        } else {
            Some(CollectionDelta {
                new_entities,
                deleted_entities,
                mutated_entities,
            })
        }
    }
    fn update(&mut self, delta: &Self::Delta) {
        if let Some(delta) = delta {
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
}

impl<T: HasId> Collection<T> {
    /// Construct an empty collection
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
        }
    }

    /// Insert a new entity into the collection
    pub fn insert(&mut self, entity: T) {
        self.by_id.insert(entity.id().clone(), entity);
    }

    /// Get an entity by its id
    pub fn get(&self, id: &T::Id) -> Option<&T> {
        self.by_id.get(id)
    }

    /// Get a mutable ref to an entity by its id
    pub fn get_mut(&mut self, id: &T::Id) -> Option<&mut T> {
        self.by_id.get_mut(id)
    }

    /// Remove an entity by its id
    pub fn remove(&mut self, id: &T::Id) -> Option<T> {
        self.by_id.remove(id)
    }

    /// Retain only entities specified by the predicate
    pub fn retain<F: FnMut(&T) -> bool>(&mut self, mut f: F) {
        self.by_id.retain(move |_, obj| f(obj));
    }

    /// Gets an iterator over the entities
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.into_iter()
    }

    /// Gets an iterator over mutable refs of the entities
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.into_iter()
    }

    /// Gets an iterator of entities' ids
    pub fn ids(&self) -> impl Iterator<Item = &T::Id> + '_ {
        self.by_id.keys()
    }

    /// Get number of entities in the collection
    pub fn len(&self) -> usize {
        self.by_id.len()
    }

    /// Check if collection is empty
    pub fn is_empty(&self) -> bool {
        self.by_id.is_empty()
    }

    /// Remove all entities from the collection
    pub fn clear(&mut self) {
        self.by_id.clear();
    }
}

impl<T: HasId> Default for Collection<T> {
    fn default() -> Self {
        Self::new()
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
        Box::new(self.by_id.into_values())
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
