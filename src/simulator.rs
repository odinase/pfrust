use crate::particles;

pub struct Simulator<P, M>
where
    P: particles::ProcessModel,
    M: particles::MeasurementModel<
        State = <P as particles::ProcessModel>::State,
        Measurement = <M as particles::MeasurementModel>::Noise,
    >,
    <M as particles::MeasurementModel>::Measurement:
        std::ops::Add<Output = <M as particles::MeasurementModel>::Measurement>,
{
    proc_model: P,
    meas_model: M,
}

impl<P, M> Simulator<P, M>
where
    P: particles::ProcessModel,
    M: particles::MeasurementModel<
        State = <P as particles::ProcessModel>::State,
        Measurement = <M as particles::MeasurementModel>::Noise,
    >,
    <M as particles::MeasurementModel>::Measurement:
        std::ops::Add<Output = <M as particles::MeasurementModel>::Measurement>,
{
    pub fn new(proc_model: P, meas_model: M) -> Self {
        Simulator {
            proc_model,
            meas_model,
        }
    }
    fn gen_measurement(
        &self,
        state: &<P as particles::ProcessModel>::State,
    ) -> <M as particles::MeasurementModel>::Measurement {
        self.meas_model.evaluate(state) + self.meas_model.sample()
    }
    fn propagate(
        &self,
        state: &<P as particles::ProcessModel>::State,
        input: &<P as particles::ProcessModel>::Input,
    ) -> <P as particles::ProcessModel>::State {
        self.proc_model.discrete(state, input)
    }
    // Consumes the simulator, yielding ground truth and noisy measurements along the trajectory
    pub fn run(
        self,
        init_state: <P as particles::ProcessModel>::State,
        controller: impl Fn(
            &<P as particles::ProcessModel>::State,
        ) -> <P as particles::ProcessModel>::Input,
        n: usize,
    ) -> (
        Vec<<P as particles::ProcessModel>::State>,
        Vec<<M as particles::MeasurementModel>::Measurement>,
    ) {
        let mut ground_truth: Vec<<P as particles::ProcessModel>::State> = Vec::with_capacity(n);
        let mut measurements: Vec<<M as particles::MeasurementModel>::Measurement> =
            Vec::with_capacity(n);
        // Initialize
        measurements.push(self.gen_measurement(&init_state));
        ground_truth.push(init_state);
        for i in 1..n {
            {
                let curr_state = &ground_truth[i - 1];
                let input = controller(curr_state);
                ground_truth.push(self.propagate(curr_state, &input));
            }
            measurements.push(self.gen_measurement(&ground_truth[i - 1]));
        }
        (ground_truth, measurements)
    }
}
