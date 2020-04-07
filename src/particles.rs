use crate::pdfs;

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

pub trait ProcessModel {
    type State;
    type Input;
    fn continuous(&self, state: &Self::State, input: &Self::Input) -> Self::State;
    fn discrete(&self, state: &Self::State, input: &Self::Input) -> Self::State;
}

pub trait MeasurementModel {
    type State;
    type Measurement;
    type Noise;
    fn evaluate(&self, state: &Self::State) -> Self::Measurement;
    fn noise_density(&self, noise: Self::Noise) -> pdfs::Result<Self::Noise>;
    fn sample(&self) -> Self::Noise;
}

pub mod pendulum;
