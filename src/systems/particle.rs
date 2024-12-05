// src/systems/particle.rs
use specs::{System, ReadStorage, Entities, Join, LazyUpdate, Read, Builder};
use crate::components::{Position, ParticleEmitter, Renderable, Velocity};
use rand::Rng;

pub struct ParticleSystem;

impl<'a> System<'a> for ParticleSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, ParticleEmitter>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (entities, positions, emitters, lazy): Self::SystemData) {
        let mut rng = rand::thread_rng();
        
        for (_entity, pos, emitter) in (&entities, &positions, &emitters).join() {
            if rng.gen::<f32>() < emitter.rate {
                let particle_pos = Position {
                    x: pos.x,
                    y: pos.y,
                };
                
                let particle_vel = Velocity {
                    x: rng.gen_range(-50.0..50.0),
                    y: rng.gen_range(-50.0..50.0),
                };
                
                lazy.create_entity(&entities)
                    .with(particle_pos)
                    .with(particle_vel)
                    .with(Renderable {
                        width: 4.0,
                        height: 4.0,
                        color: emitter.color,
                    })
                    .build();
            }
        }
    }
}