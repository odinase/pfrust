use super::Particle;
use crate::pdfs::{Pdf, Result, Triangular};
use ndarray::prelude::*;
#[derive(Clone, Debug)]
pub struct PendulumParticle {
    weight: Option<f64>,
    state: Array1<f64>,
    meas_model: MeasurementModel,
    proc_model: ProcessModel,
}

// Assumes triangular pdf
#[derive(Clone, Copy, Debug)]
pub struct MeasurementModel {
    noise_pdf: Triangular,
    Ld: f64,
    Ll: f64,
    l: f64,
}
impl MeasurementModel {
    pub fn new(min: f64, max: f64, mode: f64, Ld: f64, Ll: f64, l: f64) -> Self {
        MeasurementModel {
            noise_pdf: Triangular { min, max, mode },
            Ld,
            Ll,
            l,
        }
    }
    fn evaluate(&self, state: &Array1<f64>) -> f64 {
        let theta = state[0];
        (self.Ld - self.l * theta.cos()).hypot(self.Ll - self.l * theta.sin())
    }
    fn noise_density(&self, noise: f64) -> Result {
        self.noise_pdf.prob_density(&noise)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct ProcessModel {
    g: f64,
    l: f64,
    d: f64,
    ts: f64,
}

impl ProcessModel {
    pub fn new(g: f64, l: f64, d: f64, ts: f64) -> Self {
        ProcessModel { g, l, d, ts }
    }
    fn continuous(&self, state: &Array1<f64>, input: &Array1<f64>) -> Array1<f64> {
        arr1(&[
            state[1],
            -self.d * state[1] - self.g / self.l * state[0].sin(),
        ]) + input
    }
    // Simple forward explicit Euler
    fn discrete(&self, state: &Array1<f64>, input: &Array1<f64>) -> Array1<f64> {
        state + &(self.continuous(state, input) * self.ts)
    }
}

impl Particle for PendulumParticle {
    type State = Array1<f64>;
    type Measurement = f64;
    type Input = Array1<f64>;
    fn predict(&mut self, input: &Array1<f64>) {
        self.state = self.proc_model.discrete(&self.state, input);
    }
    fn update(&mut self, measurement: &f64) {
        let meas_noise = measurement - &self.meas_model.evaluate(&self.state);
        let pdf_val = self.meas_model.noise_density(meas_noise).ok();
        self.weight = match (self.weight, pdf_val) {
            (Some(w), Some(p)) => Some(w * p),
            _ => None,
        };
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
    pub fn new(
        init_state: Array1<f64>,
        meas_model: MeasurementModel,
        proc_model: ProcessModel,
    ) -> Self {
        PendulumParticle {
            weight: None,
            state: init_state,
            meas_model,
            proc_model,
        }
    }
}
