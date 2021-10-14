use geng::prelude::*;

#[derive(ecs::Query, Debug, PartialEq)]
struct Query<'a> {
    int: &'a i32,
}

#[test]
fn test() {
    let mut entity = ecs::Entity::new();
    entity.add(123);
    assert_eq!(*entity.query::<Query>(), Some(Query { int: &123 }));
}
