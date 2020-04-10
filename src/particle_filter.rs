use super::particles;
use rand::thread_rng;
use rand_distr::{Distribution, Uniform};

/// The particle filter struct that contains the particles of the filter and runs the algorithm. 
/// 
/// Contains a `predict` and `update` method that performs the two steps of the particle filter algorithm. 
#[derive(Debug, Clone)]
pub struct ParticleFilter<P: particles::Particle + std::fmt::Debug> {
    /// Vector over all 
    particles: Vec<P>,
}

impl<P: particles::Particle + std::fmt::Debug> ParticleFilter<P> {
    /// Returns an instance of ParticleFilter.
    /// 
    /// The new ParticleFilter instance takes ownership of a vector of particles. The particles should have a randomized state distributed uniformly. `new` will initialize the weights of the particles to all have equal weight 
    /// 
    /// # Arguments
    /// 
    /// * `init_particles` - Vector of particles with an initialized state.
    /// 
    /// # Example
    /// 
    /// ```
    /// let num_particles = 1_000;
    /// let mut init_particles: Vec<pendulum::PendulumParticle> = Vec::with_capacity(num_particles);
    /// let rand_ang_pos = Uniform::new(-FRAC_PI_2, FRAC_PI_2);
    /// let rand_ang_vel = Uniform::new(0., FRAC_PI_4);
    /// for _ in 0..num_particles {
    ///     let init_state = arr1(&[
    ///         rand_ang_pos.sample(&mut thread_rng()),
    ///         rand_ang_vel.sample(&mut thread_rng()),
    ///     ]);
    ///     let meas_model = pendulum::PendulumMeasurementModel::new(min, max, mode, ld, ll, l);
    ///     let proc_model = pendulum::PendulumProcessModel::new(g, l, d, ts);
    ///     init_particles.push(pendulum::PendulumParticle::new(
    ///         init_state, meas_model, proc_model,
    ///     ));
    /// }
    /// let mut pf = particle_filter::ParticleFilter::new(init_particles);
    /// ```
    pub fn new(init_particles: Vec<P>) -> Self {
        let mut pf = ParticleFilter {
            particles: init_particles,
        };
        pf.initialize_weights();
        pf
    }
    /// Performs the prediction step of the particle filter algorithm.
    /// 
    /// Takes in an input with the type given in the [Particle trait](../particles/trait.Particle.html).
    pub fn predict(&mut self, input: &<P as particles::Particle>::Input) {
        for particle in &mut self.particles {
            particle.predict(input);
        }
    }

    pub fn update(&mut self, measurement: &<P as particles::Particle>::Measurement) {
        for particle in &mut self.particles {
            particle.update(measurement);
        }
        self.normalize_weights();
        self.resample();
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
        // if total_weight < 1e-6 {
        //     self.initialize_weights()
        // } else {
        for particle in &mut self.particles {
            if let Some(w) = particle.get_weight() {
                particle.set_weight(Some(w / total_weight));
            } else {
                particle.set_weight(Some(0.0));
            }
        }
    }
    fn initialize_weights(&mut self) {
        let n = self.particles.len() as f64;
        for particle in &mut self.particles {
            particle.set_weight(Some(1.0 / n));
        }
    }
    // Perform systematic resampling
    fn resample(&mut self) {
        let cumsum_weights: Vec<f64> = self
            .particles
            .iter()
            .scan(0.0_f64, |cw, p| {
                let p = p.get_weight().unwrap_or_default();
                *cw = *cw + p;
                Some(*cw)
            })
            .collect();
        let n = self.particles.len();
        let rand_noise: f64 = Uniform::new(0.0, 1.0).sample(&mut thread_rng());
        // let u: Vec<f64> = (0..n).map(|x| (x as f64 + rand_noise) / n as f64).collect();
        // let (mut i, mut j) = (0, 0);
        let mut indices: Vec<usize> = vec![0; n];
        // while i < n {
        //     if u[i] < cumsum_weights[j] {
        //         indices[i] = j;
        //         i += 1;
        //     } else {
        //         j += 1;
        //     }
        // }
        let mut j = 0;
        for i in 0..n {
            let u = (i as f64 + rand_noise) / n as f64;
            while u > cumsum_weights[j] {
                j += 1;
            }
            indices[i] = j;
        }
        self.particles = (0..n)
            .map(|i| {
                let mut p = self.particles[indices[i]].clone();
                p.set_weight(Some(1. / n as f64));
                p
            })
            .collect();
    }
}
