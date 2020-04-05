use gnuplot::*;
use ndarray::prelude::*;
use ndarray::Array;
use ndarray_rand::rand_distr::Normal;
use ndarray_rand::RandomExt;
use pfrust::particle_filter::ParticleFilter;
use pfrust::particles::pendulum;
use rosrust;
use rosrust_msg;
use std::f64::consts::PI;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // println!("This is a silly example of doing an animation... Ctrl-C to quit.");
    // let mut fg = Figure::new();
    // loop {
    //     fg.clear_axes();
    //     let x = Array::random((100, 1), Normal::new(0., 1.).unwrap());
    //     let y = Array::random((100, 1), Normal::new(0., 1.).unwrap());
    //     fg.axes2d()
    //         .set_y_range(Fix(-3.0), Fix(3.0))
    //         .set_x_range(Fix(-3.0), Fix(3.0))
    //         .points(x.iter(), y.iter(), &[]);
    //     fg.show().unwrap();
    //     sleep(Duration::from_millis(50));
    // }
    let x = arr1(&[1., 2.]);
    let A = arr2(&[[1., 2.], [3., 4.]]);
    let B = arr2(&[[5., 11.], [4., 7.]]);
    println!("{}", x.dot(&A.dot(&(B.dot(&x)))));
}
