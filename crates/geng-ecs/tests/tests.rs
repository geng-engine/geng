use ecs::{Entity, World};
use geng_ecs as ecs;
use std::collections::HashSet;

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

    assert_eq!(world.query::<&i32>().iter().collect::<HashSet<_>>(), {
        let mut expected = HashSet::new();
        expected.insert(&1);
        expected.insert(&2);
        expected
    });
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
    assert_eq!(*entity.query_filtered::<&i32, ecs::Without<Flag>>(), None);
    assert_eq!(*entity.query_filtered::<&i32, ecs::With<Flag2>>(), None);
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
fn test_manual_impl() {
    #[derive(Debug, PartialEq)]
    struct Foo<'a> {
        x: &'a i32,
        y: &'a mut bool,
    }

    unsafe impl<'a> ecs::Query<'a> for Foo<'a> {
        type WorldBorrows = (
            <&'a i32 as ecs::Query<'a>>::WorldBorrows,
            <&'a mut bool as ecs::Query<'a>>::WorldBorrows,
        );
        unsafe fn borrow_world(world: &'a ecs::World) -> Option<Self::WorldBorrows> {
            let x = <&'a i32 as ecs::Query<'a>>::borrow_world(world)?;
            let y = <&'a mut bool as ecs::Query<'a>>::borrow_world(world)?;
            Some((x, y))
        }
        unsafe fn get_world(borrows: &Self::WorldBorrows, id: ecs::Id) -> Option<Self> {
            let (x, y) = borrows;
            let x = <&'a i32 as ecs::Query<'a>>::get_world(x, id)?;
            let y = <&'a mut bool as ecs::Query<'a>>::get_world(y, id)?;
            Some(Foo { x, y })
        }
        type DirectBorrows = (
            <&'a i32 as ecs::Query<'a>>::DirectBorrows,
            <&'a mut bool as ecs::Query<'a>>::DirectBorrows,
        );
        unsafe fn borrow_direct(entity: &'a Entity) -> Option<Self::DirectBorrows> {
            let x = <&'a i32 as ecs::Query<'a>>::borrow_direct(entity)?;
            let y = <&'a mut bool as ecs::Query<'a>>::borrow_direct(entity)?;
            Some((x, y))
        }
        unsafe fn get(borrows: &Self::DirectBorrows) -> Self {
            let (x, y) = borrows;
            let x = <&'a i32 as ecs::Query<'a>>::get(x);
            let y = <&'a mut bool as ecs::Query<'a>>::get(y);
            Foo { x, y }
        }
    }

    let mut entity = Entity::new();
    entity.add(42);
    entity.add(false);
    assert_eq!(
        *entity.query::<Foo>(),
        Some(Foo {
            x: &42,
            y: &mut false
        }),
    );
}

#[test]
fn test_derive() {
    #[derive(ecs::Query, Debug, PartialEq)]
    struct Foo<'a> {
        x: &'a i32,
        y: &'a mut bool,
    }

    let mut entity = Entity::new();
    entity.add(42);
    entity.add(false);
    assert_eq!(
        *entity.query::<Foo>(),
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
