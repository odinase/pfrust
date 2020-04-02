// use rand::thread_rng;
// use rand_distr::{Distribution, Triangular, TriangularError};


// fn main() -> Result<(), TriangularError> {
//     let mut rng = thread_rng();
//     let tri_pdf = Triangular::new(-5.0, 5.0, 0.0)?;
//     for _ in 0..10 {
//         println!("Sample is: {}", tri_pdf.sample(&mut rng));
//     }
//     Ok(())
// }
use pfrust::particles::{Particle, PendulumParticle};

fn measurement_model(state: &mut Vec<f64>) {}

fn process_model(state: &mut Vec<f64>) {}

fn main() {
    PendulumParticle::new(measurement_model, process_model);
}