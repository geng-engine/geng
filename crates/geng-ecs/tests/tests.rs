use ecs::{Entity, World};
use geng_ecs as ecs;
use std::{collections::HashSet, iter::FromIterator};

#[test]
fn test_entity() {
    let mut entity = Entity::new();
    entity.add(123);
    entity.add("Hello, world!");
    assert_eq!(*entity.query::<&i32>(), Some(&123));
    assert_eq!(
        *entity.query::<(&mut i32, &&str)>(),
        Some((&mut 123, &"Hello, world!"))
    );
}

#[test]
fn test_world() {
    let mut world = World::new();

    let mut entity = Entity::new();
    entity.add(1);
    entity.add("A");
    world.add(entity);

    let mut entity = Entity::new();
    entity.add(2);
    world.add(entity);

    let mut entity = Entity::new();
    entity.add("B");
    world.add(entity);

    assert_eq!(
        world.query::<&mut i32>().iter().collect::<HashSet<_>>(),
        HashSet::from_iter([&mut 1, &mut 2]),
    );
    assert_eq!(
        world.query::<&&str>().iter().collect::<HashSet<_>>(),
        HashSet::from_iter([&"A", &"B"]),
    );
    assert_eq!(
        world
            .query_filtered::<&i32, ecs::Without<&str>>()
            .iter()
            .collect::<HashSet<_>>(),
        HashSet::from_iter([&2]),
    );
    assert_eq!(
        world
            .query_filtered::<Option<&mut &str>, ecs::With<i32>>()
            .iter()
            .collect::<HashSet<_>>(),
        HashSet::from_iter([Some(&mut "A"), None]),
    );
}

#[test]
fn test_option() {
    let mut entity = Entity::new();
    entity.add(123);
    assert_eq!(*entity.query::<&i32>(), Some(&123));
    assert_eq!(
        *entity.query::<(Option<&mut i32>, Option<&&str>)>(),
        Some((Some(&mut 123), None))
    );
}

#[test]
fn test_with_without() {
    struct Flag;
    struct Flag2;
    let mut entity = Entity::new();
    entity.add(123);
    entity.add(Flag);
    assert_eq!(
        *entity.query_filtered::<&i32, ecs::With<Flag>>(),
        Some(&123)
    );
    assert_eq!(
        *entity.query::<(ecs::With<Flag>, ecs::With<Flag2>)>(),
        Some((true, false))
    );
    assert_eq!(*entity.query_filtered::<&i32, ecs::Without<Flag>>(), None);
    assert_eq!(
        entity.filter::<(ecs::With<Flag>, ecs::Without<Flag2>)>(),
        true
    );
    assert_eq!(
        entity.filter::<(ecs::Without<Flag>, ecs::Without<Flag2>)>(),
        false
    );
    assert_eq!(
        *entity.query_filtered::<&i32, ecs::Without<Flag2>>(),
        Some(&123)
    );
}

#[test]
fn test_double_borrow() {
    let mut entity = Entity::new();
    entity.add(123);
    assert_eq!(*entity.query::<(&i32, &i32)>(), Some((&123, &123)));
}

#[test]
fn test_derive() {
    #[derive(ecs::Query, Debug, PartialEq)]
    struct Foo<'a, T: ecs::Component> {
        x: &'a T,
        y: &'a mut bool,
    }

    let mut entity = Entity::new();
    entity.add(42);
    entity.add(false);
    assert_eq!(
        *entity.query::<Foo<i32>>(),
        Some(Foo {
            x: &42,
            y: &mut false
        }),
    );
}

#[test]
#[should_panic]
fn test_double_mutable_borrow() {
    let mut entity = Entity::new();
    entity.add(123);
    assert_eq!(
        *entity.query::<(&mut i32, &mut i32)>(),
        Some((&mut 123, &mut 123))
    );
}
