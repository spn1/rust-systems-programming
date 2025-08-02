use super::particle::Particle;
use rand::prelude::*; // RNG

pub struct World {
    current_turn: u64,
    pub particles: Vec<Box<Particle>>,
    pub height: f64,
    pub width: f64,
    rng: ThreadRng,
}

impl World {
    pub fn new(width: f64, height: f64) -> World {
        World {
            current_turn: 0,
            particles: Vec::<Box<Particle>>::new(),
            height: height,
            width: width,
            rng: thread_rng()
        }
    }

    pub fn add_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let particle = Particle::new(&self);
            let boxed_particle = Box::new(particle);
            self.particles.push(boxed_particle);
        }
    }

    pub fn remove_shapes(&mut self, n: i32) {
        for _ in 0..n.abs() {
            let mut to_delete = None;

            let particle_iter = self.particles
                .iter()
                .enumerate();

            for (i, particle) in particle_iter {
                if particle.color[3] < 0.02 {
                    to_delete = Some(i);
                }
                break;
            }

            if let Some(i) = to_delete {
                self.particles.remove(i);
            } else {
                self.particles.remove(0);
            }
        }
    }

    pub fn update(&mut self) {
        let n = self.rng.gen_range(-3..=3);

        if n > 0 {
            self.add_shapes(n);
        } else {
            self.remove_shapes(n);
        }

        self.particles.shrink_to_fit();
        for shape in &mut self.particles {
            shape.update();
        }

        self.current_turn += 1;
    }
}