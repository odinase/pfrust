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
use textplots::{Chart, Plot, Shape};
use std::process::Command;
use pfrust::particles;

fn main() {
        Chart::default().lineplot( Shape::Continuous( |x| x.exp())).display();
}