/** General trait for methods of Particles:
 * predict()
 * update()
 */
pub trait Particle<F1: Fn(&mut Vec<f64>), F2: Fn(&mut Vec<f64>)> {
    /**
    * Create new particle. Right now takes in a generic vector, but probably best as the state is very generic.
    * Static is chosen as lifetime as the measurement model should be a "normal function".
    */
    fn new(process_model: F1, measurement_model: F2) -> Self;
    /**
    * Propagates the particle according to the process model, assuming no noise
    */
    fn predict(&mut self);
    /**
    * Runs full update, where the weights of the particle is updated according to the measurement model
    */
    fn update(&mut self);
}

pub struct PendulumParticle<F1, F2> {
    process_model: F1,
    measurement_model: F2,
    weight: Option<f64>
}

impl<F1: Fn(&mut Vec<f64>), F2: Fn(&mut Vec<f64>)> Particle<F1, F2> for PendulumParticle<F1, F2> {
    fn new(process_model: F1, measurement_model: F2) -> Self {
        PendulumParticle{
            process_model,
            measurement_model,
            weight: None,
        }
    }
    fn predict(&mut self) {}
    fn update(&mut self) {}
}