pub trait Particle {
    type State;
    type Measurement;
    type Input;
    type ParticleWeight;
    /**
     * Propagates the particle according to the process model, assuming no noise.
     */
    fn predict(&mut self, input: &Self::Input);
    /**
     * Runs full update, where the weights of the particle is updated according to the measurement model
     */
    fn update(&mut self, measurement: &Self::Measurement);
    fn get_weight(&self) -> Self::ParticleWeight;
    fn set_weight(&mut self, new_weight: Self::ParticleWeight);
    fn get_state<'a>(&'a self) -> &'a Self::State;
}

mod pendulum {

    use super::Particle;
    use crate::pdfs::{Triangular, Pdf, Result};
    use ndarray::prelude::*;


    pub struct PendulumParticle {
        weight: Option<f64>,
        state: Array1<f64>,
        meas_model: MeasurementModel,
        proc_model: ProcessModel,
    }

    // Assumes triangular pdf
    pub struct MeasurementModel {
        noise_pdf: Triangular,
        Ld: f64,
        Ll: f64,
        l: f64,
    }
    impl MeasurementModel {
        fn new(min: f64, max: f64, mode: f64, Ld: f64, Ll: f64, l: f64) -> Self {
            MeasurementModel{
                noise_pdf: Triangular{
                    min,
                    max,
                    mode,
                },
                Ld,
                Ll,
                l,
            }
        }
        fn evaluate(&self, state: &Array1<f64>) -> f64 {
            let theta = state[0];
            (self.Ld - self.l*theta.cos()).hypot(self.Ll - self.l*theta.sin())
        }
        fn noise_density(&self, noise: f64) -> Result {
            self.noise_pdf.prob_density(&noise)
        }
    }

    pub struct ProcessModel {
        g: f64,
        l: f64,
        d: f64,
        Ts: f64,
    }

    impl ProcessModel {
        fn continuous(&self, state: &Array1<f64>, input: &Array1<f64>) -> Array1<f64> {
            arr1(&[state[1], -self.d*state[1] - self.g/self.l*state[0].sin()]) + input
        }
        fn discrete(&self, state: &Array1<f64>, input: &Array1<f64>) -> Array1<f64> {
            state + &(self.continuous(state, input)*self.Ts)
        }
    }

    impl Particle for PendulumParticle {
        type State = Array1<f64>;
        type Measurement = f64;
        type ParticleWeight = Option<f64>;
        type Input = Array1<f64>; 
        fn predict(&mut self, input: &Array1<f64>) {
            self.proc_model.discrete(&self.state, input);
        }
        fn update(&mut self, measurement: &f64) {
            let meas_noise = measurement - &self.meas_model.evaluate(&self.state);
            self.weight = self.meas_model.noise_density(meas_noise).ok();
        }
        fn get_weight(&self) -> Option<f64> {
            self.weight
        }
        fn set_weight(&mut self, new_weight: Option<f64>) {
            self.weight = new_weight;
        }
        fn get_state<'a>(&'a self) -> &'a Array1<f64> {
            &self.state
        }
    }
    impl PendulumParticle {
        fn new(init_state: Array1<f64>, meas_model: MeasurementModel, proc_model: ProcessModel) -> Self {
            PendulumParticle {
                weight: None,
                state: init_state,
                meas_model,
                proc_model,
            }
        }
    }
}
