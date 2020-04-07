use gnuplot::*;
use ndarray::prelude::*;
use pfrust::simulator::Simulator;
use pfrust::particles::pendulum;
use std::f64::consts::PI;
// use ndarray_rand::rand_distr::Normal;
// use ndarray_rand::RandomExt;
// use pfrust::particle_filter::ParticleFilter;
// use pfrust::particles::pendulum;
// use rosrust;
// use rosrust_msg;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // println!("This is a silly example of doing an animation... Ctrl-C to quit.");
    let init_state = arr1(&[PI / 2., -PI / 100.]);
    let controller = |_ : &Array1<f64>| arr1(&[0., 0.]);
    let (min, max, mode) = (-0.25, 0.25, 0.0);
    let (ld, ll, l) = (4., 0., 1.);
    let pmm = pendulum::PendulumMeasurementModel::new(min, max, mode, ld, ll, l);
    let (g, d, ts) = (9.81, 0., 0.05);
    let ppm = pendulum::PendulumProcessModel::new(g, l, d, ts);
    let sim = Simulator::new(ppm, pmm);
    let n = 800;
    let (gt, m) = sim.run(init_state, controller, n);
    let mut fg = Figure::new();
    for p in gt {
        fg.clear_axes();
        fg.axes2d()
            .set_y_range(Fix(-5.0), Fix(5.0))
            .set_x_range(Fix(-5.0), Fix(5.0))
            .points(arr0(l*p[0].cos()).iter(), arr0(l*p[0].sin()).iter(), &[]);
        fg.show().unwrap();
        sleep(Duration::from_millis(50));
    }
}
