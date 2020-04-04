use pfrust::particles::pendulum;
use pfrust::particle_filter::ParticleFilter;
use ndarray::prelude::*;

fn main() {
    let mm = pendulum::MeasurementModel::new(-5.0, 5.0, 0.0, 0.1, 0.1, 1.0);
    let pm = pendulum::ProcessModel::new(9.81, 1.0, 0.0, 1.0_f64);
    let init_state : Array1<f64> = arr1(&[0.0, 0.0]);
    let p = pendulum::PendulumParticle::new(init_state, mm, pm);
    let p_vec : Vec<pendulum::PendulumParticle> = (0..10).map(|_| p.clone()).collect();
    println!("p_vec: {:#?}", p_vec);
    let mut pf = ParticleFilter::new(p_vec);
    println!("pf {:#?}", pf);
    let input: Array1<f64> = arr1(&[0.0, 0.1]);
    let measurement: f64 = 0.5;
    pf.predict(&input);
    pf.update(&measurement);
    println!("pf {:#?}", pf);
}
