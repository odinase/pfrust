use custom_error::custom_error;
use ndarray::prelude::*;
use ndarray_linalg::error::LinalgError;
use ndarray_linalg::solve::{Determinant, Solve};
use std::f64::consts::PI;
use rand_distr::{Distribution};
use rand_distr;
use rand::thread_rng;

pub type Result<T> = std::result::Result<T, PdfError>;

pub trait Pdf<T> {
    type Realization;
    fn prob_density(&self, x: &Self::Realization) -> Result<T>;
}

custom_error! {pub PdfError
    LinAlgError {source: LinalgError} = "Tried executing invalid linalg operation",
    OutOfBounds {tried_val: f64, bound: f64} = "Value {tried_val} was outside bound of {bound}",
    InvalidBounds {min: f64, max: f64}  = "Invalid bounds of min {min} and max {max} used",
}
// Apparently, there are no multivariate Gaussian crates out there for Rust???
pub fn multivariate_gauss_pdf(x: &Array1<f64>, mu: &Array1<f64>, sigma: &Array2<f64>) -> Result<f64> {
    let n = x.len() as f64;
    let y = x - mu;
    let q = y.dot(&sigma.solve(&y)?);
    let pdf_val = (2.0 * PI).powf(-n / 2.0) * sigma.det()?.powf(-0.5) * (-0.5 * q).exp();
    Ok(pdf_val)
}
#[derive(Clone, Copy, Debug)]
pub struct Triangular {
    density: rand_distr::Triangular<f64>,
    pub max: f64,
    pub min: f64,
    pub mode: f64,
}

impl Triangular {
    pub fn new(min: f64, max: f64, mode: f64) -> Self {
        Triangular{
            min,
            max,
            mode,
            density: rand_distr::Triangular::new(min, max, mode).unwrap(),
        }
    }
    pub fn sample(&self) -> f64 {
        self.density.sample(&mut thread_rng())
    }
}

impl Pdf<f64> for Triangular {   
    type Realization = f64;
    fn prob_density(&self, x: &f64) -> Result<f64> {
        let x = x.clone();
        if (self.max - self.min) < 1e-6 {
            return Err(PdfError::InvalidBounds { min: self.min, max: self.max });
        } else if x < self.min {
            return Err(PdfError::OutOfBounds {
                tried_val: x,
                bound: self.min,
            });
        } else if x > self.max {
            return Err(PdfError::OutOfBounds {
                tried_val: x,
                bound: self.max,
            });
        }
        let h = 2.0 / (self.max - self.min); // For the PDF to integrate to 1
        let pdf_val = if x < self.mode {
            h / (self.mode - self.min) * (x - self.min)
        } else {
            h / (self.max - self.mode) * (self.max - x)
        };
        Ok(pdf_val)
    }
}