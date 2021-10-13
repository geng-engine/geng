use ecs::Entity;
use geng_ecs as ecs;

#[test]
fn test() {
    let mut entity = Entity::new();
    entity.add(123i32);
    entity.add("Hello, world!");
    assert_eq!(*entity.query::<&i32>(), Some(&123));
    assert_eq!(
        *entity.query::<(&mut i32, &&str)>(),
        Some((&mut 123, &"Hello, world!"))
    );
}

#[test]
fn test_option() {
    let mut entity = Entity::new();
    entity.add(123i32);
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
    entity.add(123i32);
    entity.add(Flag);
    assert_eq!(*entity.query::<(&i32, ecs::With<Flag>)>(), Some((&123, ())));
    assert_eq!(*entity.query::<(&i32, ecs::Without<Flag>)>(), None);
    assert_eq!(*entity.query::<(&i32, ecs::With<Flag2>)>(), None);
    assert_eq!(
        *entity.query::<(&i32, ecs::Without<Flag2>)>(),
        Some((&123, ()))
    );
}

#[test]
fn test_double_borrow() {
    let mut entity = Entity::new();
    entity.add(123i32);
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
        type Output = Self;
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
    entity.add(42i32);
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
    entity.add(42i32);
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
    entity.add(123i32);
    assert_eq!(
        *entity.query::<(&mut i32, &mut i32)>(),
        Some((&mut 123, &mut 123))
    );
}
