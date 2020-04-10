use super::{MeasurementModel, Particle, ProcessModel};
use crate::pdfs::{Pdf, Result, Triangular};
use ndarray::prelude::*;
use std::f64::consts::PI;
#[derive(Clone, Debug)]
pub struct PendulumParticle {
    weight: Option<f64>,
    state: Array1<f64>,
    meas_model: PendulumMeasurementModel,
    proc_model: PendulumProcessModel,
}

// Assumes triangular pdf
#[derive(Clone, Copy, Debug)]
pub struct PendulumMeasurementModel {
    noise_pdf: Triangular,
    ld: f64,
    ll: f64,
    l: f64,
}
impl PendulumParticle {
    pub fn new(
        init_state: Array1<f64>,
        meas_model: PendulumMeasurementModel,
        proc_model: PendulumProcessModel,
    ) -> Self {
        PendulumParticle {
            weight: None,
            state: init_state,
            meas_model,
            proc_model,
        }
    }
}
impl PendulumMeasurementModel {
    pub fn new(min: f64, max: f64, mode: f64, ld: f64, ll: f64, l: f64) -> Self {
        PendulumMeasurementModel {
            noise_pdf: Triangular::new(min, max, mode),
            ld,
            ll,
            l,
        }
    }
}
impl MeasurementModel for PendulumMeasurementModel {
    type State = Array1<f64>;
    type Measurement = f64;
    type Noise = f64;
    fn evaluate(&self, state: &Array1<f64>) -> f64 {
        let theta = state[0];
        (self.ld - self.l * theta.cos()).hypot(self.ll - self.l * theta.sin())
    }
    fn noise_density(&self, noise: f64) -> Result<f64> {
        self.noise_pdf.prob_density(&noise)
    }
    fn sample(&self) -> f64 {
        self.noise_pdf.sample()
    }
}
#[derive(Clone, Copy, Debug)]
pub struct PendulumProcessModel {
    g: f64,
    l: f64,
    d: f64,
    ts: f64,
}

impl PendulumProcessModel {
    pub fn new(g: f64, l: f64, d: f64, ts: f64) -> Self {
        PendulumProcessModel { g, l, d, ts }
    }
}
impl ProcessModel for PendulumProcessModel {
    type State = Array1<f64>;
    type Input = Array1<f64>;
    fn continuous(&self, state: &Array1<f64>, input: &Array1<f64>) -> Array1<f64> {
        arr1(&[
            state[1],
            -self.d * state[1] - self.g / self.l * state[0].sin(),
        ]) + input
    }
    // Simple forward explicit Euler
    fn discrete(&self, state: &Array1<f64>, input: &Array1<f64>) -> Array1<f64> {
        modulo2pi(state + &(self.continuous(state, input) * self.ts))
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
        self.weight = match self.meas_model.noise_density(meas_noise).ok() {
            Some(p) => Some(p),
            None => Some(0.0),
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
// Helpers
fn modulo2pi(a: Array1<f64>) -> Array1<f64> {
arr1(&[modulo(a[0] + PI, 2.*PI) - PI, a[1]])
}
fn modulo(lhs: f64, rhs: f64) -> f64 {
    if rhs.signum() == 0.0 {
        lhs
    } else if lhs.signum()*rhs.signum() > 0.0 {
        lhs % rhs
    } else {
        (lhs % rhs) + rhs
    }
}