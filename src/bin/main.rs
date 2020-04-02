use rand::thread_rng;
use rand_distr::{Distribution, Triangular, TriangularError};


// fn main() -> Result<(), TriangularError> {
//     let mut rng = thread_rng();
//     let tri_pdf = Triangular::new(-5.0, 5.0, 0.0)?;
//     for _ in 0..10 {
//         println!("Sample is: {}", tri_pdf.sample(&mut rng));
//     }
//     Ok(())
// }

fn func() -> i32 {
    3
}

fn test_closures(func: impl FnOnce() -> i32) -> i32 {
    return func();
}

fn main() {
    println!("func gives: {}", test_closures(func));    
    println!("func gives: {}", test_closures(func));    
}