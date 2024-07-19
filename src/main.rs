#![feature(auto_traits)]

use std::marker::PhantomData;

use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{Added, Changed, QueryBuilder},
    schedule::{IntoSystemConfigs, Schedule},
    system::Query,
    world::World,
};
use bevy_utils::tracing::Instrument;

#[derive(Component, Debug)]
struct Health;
#[derive(Component, Debug)]
struct Vel(u8);
#[derive(Component)]
struct Pig;
#[derive(Component)]
struct Zombie;
//#[derive(Component)]
struct Pos;

fn main() {
    let mut world = World::new();

    world.spawn((Vel(5), Health));

    let mut query_provider =
        QueryBuilder::<(&mut Vel, &Health), Added<Vel>>::new(&mut world).build();

    dbg!(query_provider.component_access());

    let query = unsafe {
        query_provider
            .get_single_unchecked(world.as_unsafe_world_cell())
            .unwrap()
    };
    dbg!(query.1);
    dbg!(query.0);
}

auto trait Test {}
