use gnuplot::*;
use ndarray::{prelude::*, s};
use pfrust::particle_filter;
use pfrust::particles::pendulum;
use pfrust::particles::Particle;
use pfrust::simulator::Simulator;
use rand::thread_rng;
use rand_distr::{Distribution, Uniform};
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let init_state = arr1(&[PI / 2., -PI / 100.]);
    let controller = |_: &Array1<f64>| arr1(&[0., 0.]);
    let (min, max, mode) = (-0.25, 0.25, 0.0);
    let (ld, ll, l) = (4., 0., 1.);
    let (g, d, ts) = (9.81, 0., 0.05);
    let pmm = pendulum::PendulumMeasurementModel::new(min, max, mode, ld, ll, l);
    let ppm = pendulum::PendulumProcessModel::new(g, l, d, ts);
    let sim = Simulator::new(ppm, pmm);
    let n = 800;
    let (ground_truth, measurements) = sim.run(init_state, controller, n);
    // Initialize all particles
    let num_particles = 1_000;
    let mut init_particles: Vec<pendulum::PendulumParticle> = Vec::with_capacity(num_particles);
    let rand_ang_pos = Uniform::new(-FRAC_PI_2, FRAC_PI_2);
    let rand_ang_vel = Uniform::new(0., FRAC_PI_4);
    for _ in 0..num_particles {
        let init_state = arr1(&[
            rand_ang_pos.sample(&mut thread_rng()),
            rand_ang_vel.sample(&mut thread_rng()),
        ]);
        let meas_model = pendulum::PendulumMeasurementModel::new(min, max, mode, ld, ll, l);
        let proc_model = pendulum::PendulumProcessModel::new(g, l, d, ts);
        init_particles.push(pendulum::PendulumParticle::new(
            init_state, meas_model, proc_model,
        ));
    }
    let mut pf = particle_filter::ParticleFilter::new(init_particles);
    let mut fg = Figure::new();
    for (i, measurement) in measurements.iter().enumerate() {
        pf.update(measurement);
        fg.clear_axes();
        fg.axes2d()
            .set_y_range(Fix(-l * 1.5), Fix(l * 1.5))
            .set_x_range(Fix(-l * 1.5), Fix(l * 1.5))
            .points(
                pf.get_particles().iter().map(|p| {
                    let s = p.get_state();
                    l * s[0].sin()
                }),
                pf.get_particles().iter().map(|p| {
                    let s = p.get_state();
                    -l * s[0].cos()
                }),
                &[Caption("Particles"), PointSymbol('x')],
            )
            .points(
                ground_truth[i].slice(s![0]).mapv(|p| l * p.sin()).iter(),
                ground_truth[i].slice(s![0]).mapv(|p| -l * p.cos()).iter(),
                &[Caption("Ground truth"), PointSymbol('O')],
            );
        fg.show().unwrap();
        sleep(Duration::from_millis((ts*1e3) as u64));
        pf.predict(&arr1(&[0., 0.]));
    }
}
