use super::particles;


pub struct ParticleFilter<P: particles::Particle>
{
    particles: Vec<P>,
}

impl<P: particles::Particle> ParticleFilter<P>
{
    fn predict(&mut self) {
        for particle in &mut self.particles {
            particle.predict();
        }
    }

    fn update(&mut self) {

    }
}
