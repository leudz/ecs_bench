#![feature(test)]
extern crate test;
use test::Bencher;

#[macro_use]
extern crate trex;

use trex::{ComponentFilter, EventEmitter, EventQueue, Simulation, System, World};

extern crate ecs_bench;
use ecs_bench::pos_vel::{Position, Velocity, N_POS, N_POS_PER_VEL};

pub struct PosComp(Position);
pub struct VelComp(Velocity);

components!(PosComp, VelComp);

pub struct PhysicsSystem {
    filter: ComponentFilter,
}

impl PhysicsSystem {
    pub fn new() -> PhysicsSystem {
        PhysicsSystem {
            filter: ComponentFilter::new().with::<PosComp>().with::<VelComp>(),
        }
    }
}

impl System for PhysicsSystem {
    fn update(
        &mut self,
        world: &mut World,
        _queue: &EventQueue,
        _emitter: &mut EventEmitter,
        _dt: f32,
    ) {
        for entity in world.filter(&self.filter) {
            let &VelComp(Velocity { dx, dy }) = world.get(entity).unwrap();
            let mut pos = world.get_mut::<PosComp>(entity).unwrap();
            pos.0.x += dx;
            pos.0.y += dy;
        }
    }
}

pub struct RenderSystem {
    filter: ComponentFilter,
}

impl RenderSystem {
    pub fn new() -> RenderSystem {
        RenderSystem {
            filter: ComponentFilter::new().with::<PosComp>(),
        }
    }
}

impl System for RenderSystem {
    fn update(
        &mut self,
        world: &mut World,
        _queue: &EventQueue,
        _emitter: &mut EventEmitter,
        _dt: f32,
    ) {
        for entity in world.filter(&self.filter) {
            world.get::<PosComp>(entity);
        }
    }
}

fn build() -> Simulation {
    let world = {
        let mut world = World::new();
        world.register::<PosComp>();
        world.register::<VelComp>();

        for i in 0..N_POS {
            let entity = world.create();
            world.add(entity, PosComp(Position { x: 0.0, y: 0.0 }));
            if i % N_POS_PER_VEL == 0 {
                world.add(entity, VelComp(Velocity { dx: 0.0, dy: 0.0 }));
            }
        }

        world
    };

    let queue = EventQueue::new();
    let emitter = EventEmitter::new();

    let mut simulation = Simulation::new(world, queue, emitter);
    simulation.register(PhysicsSystem::new());
    simulation.register(RenderSystem::new());
    simulation
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut simulation = build();

    b.iter(|| {
        simulation.update(1.0);
    });
}
