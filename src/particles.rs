pub trait Particle: Clone {
    type State;
    type Measurement;
    type Input;
    /**
     * Propagates the particle according to the process model, assuming no noise.
     */
    fn predict(&mut self, input: &Self::Input);
    /**
     * Runs full update, where the weights of the particle is updated according to the measurement model
     */
    fn update(&mut self, measurement: &Self::Measurement);
    fn get_weight(&self) -> Option<f64>;
    fn set_weight(&mut self, new_weight: Option<f64>);
    fn get_state<'a>(&'a self) -> &'a Self::State;
}

pub mod pendulum;
