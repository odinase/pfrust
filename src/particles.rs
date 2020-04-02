/** General trait for methods of Particles:
 * predict()
 * update()
 */
pub trait Particle {
    /**
    * Create new particle
    */
    fn new(measurement_model: impl Fn() ->) -> Self;
    /**
    * Propagates the particle according to the process model, assuming no noise
    */
    fn predict(&mut self);
    /**
    * Runs full update, where the weights of the particle is updated according to the measurement model
    */
    fn update(&mut self);
}

pub struct PendulumParticle {}

impl Particle for PendulumParticle {
    fn new() -> PendulumParticle {PendulumParticle{}}
    fn predict(&mut self) {}
    fn update(&mut self) {}
}