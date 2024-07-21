#![feature(auto_traits)]

use std::marker::PhantomData;

use bevy_ecs::{
    change_detection::{DetectChanges, MAX_CHANGE_AGE},
    component::{Component, Tick},
    entity::Entity,
    query::{Added, Changed, QueryBuilder},
    schedule::{IntoSystemConfigs, Schedule},
    system::Query,
    world::{Mut, Ref, World, CHECK_TICK_THRESHOLD},
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
    let entities = world.spawn(Vel(0)).id();

    let mut query_changed = world.query_filtered::<&Vel, Changed<Vel>>();
    let mut query = world.query::<&mut Vel>();

    dbg!(query_changed.iter(&world).count());

    world.clear_trackers();
    query.get_mut(&mut world, entities).unwrap().0 += 1;
    dbg!(query_changed.iter(&world).count());
}

auto trait Test {}
