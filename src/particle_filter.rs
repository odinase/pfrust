use super::particles;
use rand::thread_rng;
use rand_distr::{Distribution, Uniform};

#[derive(Debug, Clone)]
pub struct ParticleFilter<P: particles::Particle + std::fmt::Debug> {
    particles: Vec<P>,
}

impl<P: particles::Particle + std::fmt::Debug> ParticleFilter<P> {
    pub fn new(init_particles: Vec<P>) -> Self {
        let mut pf = ParticleFilter {
            particles: init_particles,
        };
        pf.initialize_weights();
        pf
    }
    pub fn predict(&mut self, input: &<P as particles::Particle>::Input) {
        for particle in &mut self.particles {
            particle.predict(input);
        }
    }

    pub fn update(&mut self, measurement: &<P as particles::Particle>::Measurement) {
        for particle in &mut self.particles {
            particle.update(measurement);
        }
        self.resample();
        self.normalize_weights();
    }
    pub fn get_particles<'a>(&'a self) -> &'a Vec<P> {
        &self.particles
    }
    fn normalize_weights(&mut self) {
        let total_weight: f64 = self
            .particles
            .iter()
            .map(|p| p.get_weight().unwrap_or_default()) // default of f64 is 0.0, so sums correctly.
            .sum();
        for particle in &mut self.particles {
            if let Some(w) = particle.get_weight() {
                particle.set_weight(Some(w / total_weight));
            }
        }
    }
    fn initialize_weights(&mut self) {
        let n = self.particles.len() as f64;
        for particle in &mut self.particles {
            particle.set_weight(Some(1.0/n));
        }
    }
    // Preform systematic resampling
    fn resample(&mut self) {
        let mut cumsum_weights: Vec<f64> = self
            .particles
            .iter()
            .scan(0.0_f64, |cw, p| {
                let p = p.get_weight().unwrap_or_default();
                *cw = *cw + p;
                Some(*cw)
            })
            .collect();
        std::dbg!(&cumsum_weights);
        // if cumsum_weights[self.particles.len() - 1] == 0.0 {
        //     std::dbg!(&self.particles);
        //     panic!("Invalid cum!");
        // }
        cumsum_weights[self.particles.len() - 1] = 1.0;
        let cumsum_weights = cumsum_weights;
        let n = self.particles.len();
        let rand_noise: f64 = Uniform::new(0.0, 1.0).sample(&mut thread_rng());
        let u : Vec<f64> = (0..n).map(|x| (x as f64 + rand_noise) / n as f64).collect();
        std::dbg!(&u);
        let (mut i, mut j) = (0, 0);
        let mut indices : Vec<usize> = vec![0; n];
        while i < n {
            if u[i] < cumsum_weights[j] {
                indices[i] = j;
                i += 1;
            } else {
                j += 1;
            }
        }
        // TODO: Is this slow?
        self.particles = (0..n).map(|i| {
            let mut p = self.particles[indices[i]].clone();
            p.set_weight(Some(1. / n as f64));
            p
        }).collect();

    }
}
