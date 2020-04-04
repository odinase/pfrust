use custom_error::custom_error;
use ndarray::prelude::*;
use ndarray_linalg::error::LinalgError;
use ndarray_linalg::solve::{Determinant, Solve};
use std::f64::consts::PI;

pub type Result = std::result::Result<f64, PdfError>;

pub trait Pdf {
    type Realization;
    fn prob_density(&self, x: &Self::Realization) -> Result;
}

custom_error! {pub PdfError
    LinAlgError {source: LinalgError} = "Tried executing invalid linalg operation",
    OutOfBounds {tried_val: f64, bound: f64} = "Value {tried_val} was outside bound of {bound}",
    InvalidBounds {min: f64, max: f64}  = "Invalid bounds of min {min} and max {max} used",
}
// Apparently, there are no multivariate Gaussian crates out there for Rust???
pub fn multivariate_gauss_pdf(x: &Array1<f64>, mu: &Array1<f64>, sigma: &Array2<f64>) -> Result {
    let n = x.len() as f64;
    let y = x - mu;
    let q = y.dot(&sigma.solve(&y)?);
    let pdf_val = (2.0 * PI).powf(-n / 2.0) * sigma.det()?.powf(-0.5) * (-0.5 * q).exp();
    Ok(pdf_val)
}

pub struct Triangular {
    pub max: f64,
    pub min: f64,
    pub mode: f64,
}

impl Pdf for Triangular {   
    type Realization = f64;
    fn prob_density(&self, x: &f64) -> Result {
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