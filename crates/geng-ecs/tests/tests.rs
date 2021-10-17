use ecs::*;
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
    world.spawn(entity);

    let mut entity = Entity::new();
    entity.add(2);
    world.spawn(entity);

    let mut entity = Entity::new();
    entity.add("B");
    world.spawn(entity);

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
            .filter(without::<&str>())
            .query::<&i32>()
            .iter()
            .collect::<HashSet<_>>(),
        HashSet::from_iter([&2]),
    );
    assert_eq!(
        world
            .filter(with::<i32>())
            .filter(with::<i32>())
            .query::<Option<&mut &str>>()
            .iter()
            .collect::<HashSet<_>>(),
        HashSet::from_iter([Some(&mut "A"), None]),
    );
}

#[test]
fn test_simultanious_queries() {
    let mut world = World::new();

    let mut entity = Entity::new();
    entity.add(1);
    entity.add("A");
    world.spawn(entity);

    let mut entity = Entity::new();
    entity.add(2);
    world.spawn(entity);

    let mut entity = Entity::new();
    entity.add("B");
    world.spawn(entity);

    let mut ints = world.query::<&mut i32>();
    assert_eq!(
        ints.iter().collect::<HashSet<_>>(),
        HashSet::from_iter([&mut 1, &mut 2]),
    );

    // This should not compile, since this creates multiple mutable refs
    //
    // let i1 = ints.iter().collect::<HashSet<_>>();
    // let i2 = ints.iter().collect::<HashSet<_>>();
    // println!("{:?}, {:?}", i1, i2);

    let mut strs = world.query::<&mut &str>();
    assert_eq!(
        strs.iter().collect::<HashSet<_>>(),
        HashSet::from_iter([&mut "A", &mut "B"]),
    );

    std::mem::drop((ints, strs));

    let mut _q1 = world.query::<&i32>();
    let mut _q2 = world.query::<(&i32, &mut &str)>();
}

#[test]
#[should_panic]
fn test_incorrect_simultanious_queries() {
    let mut world = World::new();

    let mut entity = Entity::new();
    entity.add(1);
    entity.add("A");
    world.spawn(entity);

    let mut entity = Entity::new();
    entity.add(2);
    world.spawn(entity);

    let mut entity = Entity::new();
    entity.add("B");
    world.spawn(entity);

    let mut ints = world.query::<&mut i32>();
    assert_eq!(
        ints.iter().collect::<HashSet<_>>(),
        HashSet::from_iter([&mut 1, &mut 2]),
    );
    let mut strs = world.query::<(&mut &str, &mut i32)>();
    assert_eq!(
        strs.iter().collect::<HashSet<_>>(),
        HashSet::from_iter([(&mut "A", &mut 1)]),
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
    assert_eq!(*entity.filter(with::<Flag>()).query::<&i32>(), Some(&123));
    assert_eq!(
        *entity.query::<(Is<With<Flag>>, Is<With<Flag2>>)>(),
        Some((true, false))
    );
    assert_eq!(*entity.filter(without::<Flag>()).query::<&i32>(), None);
    assert_eq!(entity.is((with::<Flag>(), without::<Flag2>())), true);
    assert_eq!(entity.is((without::<Flag>(), without::<Flag2>())), false);
    assert_eq!(
        *entity.filter(without::<Flag2>()).query::<&i32>(),
        Some(&123)
    );
}

#[test]
fn test_eq() {
    let mut world = World::new();

    let mut entity = Entity::new();
    entity.add(1);

    assert_eq!(entity.is(equal(1)), true);
    assert_eq!(entity.is(equal(2)), false);
    assert_eq!(entity.is(equal(1) & equal(2)), false);
    assert_eq!(
        entity.is((equal(1) | equal(2)) & with::<i32>() & without::<String>()),
        true
    );

    world.spawn(entity);

    let mut entity = Entity::new();
    entity.add(2);
    world.spawn(entity);

    let mut entity = Entity::new();
    entity.add(3);
    world.spawn(entity);

    assert_eq!(
        world
            .filter(!equal(2))
            .query::<&i32>()
            .iter()
            .collect::<HashSet<_>>(),
        HashSet::from_iter([&1, &3])
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
    #[derive(Query, Debug, PartialEq)]
    struct Foo<'a, T: Component> {
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

#[test]
fn test_remove() {
    let mut world = World::new();

    let mut entity = Entity::new();
    entity.add(1);
    entity.add("One");
    world.spawn(entity);

    let mut entity = Entity::new();
    entity.add(2);
    entity.add("Two");
    world.spawn(entity);

    let entity = world.remove(equal(2)).next().unwrap();
    assert_eq!(*entity.query::<&&str>(), Some(&"Two"));
}
