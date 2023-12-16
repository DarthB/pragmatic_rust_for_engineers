use std::f64::consts::E;

use ode_solvers::{Vector6, System, Dopri5};

use crate::v2_hints::*;
use crate::configuration::{Catalyst, HaberBoschBedSetup, HaberBoschInstance};

pub type State = Vector6<f64>; // 5 mixture components and temperature [nitrogn, hydrogen, ammonia, argon, metan, temperature]

pub fn sequential_simulation(inst: &mut HaberBoschInstance, verbose: bool) {
    let idx:usize = todo!("loop over beds"); {
        let solver_info: HaberBoschSolverInfo = todo!{"access solver setup"};

        if verbose {
            println!(
                "Simulation of bed {} at {} with starting point {} with model:\n{:?}\n",
                idx + 1,
                solver_info.x0,
                solver_info.y0,
                solver_info.model
            );
        }

        // we need Dopri5 because it supports solout()
        let mut stepper = Dopri5::new(
            solver_info.model,
            solver_info.x0,
            solver_info.x0 + 25.,
            25.0 / 2000.,
            solver_info.y0,
            10e-12,
            10e-16,
        );

        //let mut stepper = Rk4::new(model, 0.0, y0, 25., 25. / 2000.);
        let res = stepper.integrate();
        match res {
            Ok(stats) => {
                if verbose {
                    println! {"{}", stats};
                }
            }
            Err(e) => println!("Error: {}", e),
        }

        let x_out = stepper.x_out();
        let y_out = stepper.y_out();

        todo!{"Store results"};
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct HaberBoschModel {
    pub pressure: f64,
    pub alpha: f64,
    pub ea: f64,
    pub big_a: f64,
    pub beta: f64,
    pub t_slope: f64,
    pub t_max: f64,
}

/// This type is the answer to:
/// What data is needed by the ODE-solver and how can we provide it?
#[derive(Debug, Copy, Clone)]
pub struct HaberBoschSolverInfo {
    pub model: HaberBoschModel,
    pub x0: f64,
    pub y0: State,
}


pub const R: f64 = 1.987; // cal/(mol·K) (GAS CONSTANT changed unit compared to video 1)

impl HaberBoschModel {
    pub fn new(pressure: f64, catalyst: Catalyst, bed: HaberBoschBedSetup) -> Self {
        match catalyst {
            Catalyst::FN => HaberBoschModel {
                pressure,
                ea: FN_EA,
                big_a: FN_BIG_A,
                beta: bed.beta,
                t_slope: bed.t_slope,
                t_max: bed.t_max,
                alpha: FN_ALPHA,
            },
            Catalyst::KMIR => HaberBoschModel {
                pressure,
                ea: KMIR_EA,
                big_a: KMIR_BIG_A,
                beta: bed.beta,
                t_slope: bed.t_slope,
                t_max: bed.t_max,
                alpha: KMIR_ALPHA,
            },
        }
    }
}

impl System<State> for HaberBoschModel {
    fn system(&self, _x: f64, y: &State, dy: &mut State) {
        // use named variables
        let (n2, h2, nh3) = (y[0], y[1], y[2]);
        // y[3] and y[4] are inerts they don't change.
        let temp = y[5];

        // intermediate calculations for reaction rates
        let k = self.big_a * (-self.ea / (R * temp)).exp();
        let log10_ka = self.beta * temp.log10() - 5.519265e-5 * temp
            + 1.848863e-7 * temp.powi(2)
            + (2001.6 / temp)
            + 2.6899;
        let ka = 10f64.powf(log10_ka);

        let t1 = ka.powi(2) * n2 * (h2.powi(3) / nh3.powi(2)).powf(self.alpha);
        let t2 = (nh3.powi(2) / h2.powi(3)).powf(1. - self.alpha);

        // reaction rates based on provided model
        let rnh3 = k * (t1 - t2);
        let rn2 = rnh3 / 2.;
        let rh2 = rnh3 / 2. * 3.;

        // derivatives for usage in the ODE solver
        // components:
        dy[0] = -rn2;
        dy[1] = -rh2;
        dy[2] = rnh3;
        dy[3] = 0.;
        dy[4] = 0.;

        // temperature
        dy[5] = self.t_slope;
        dy[5] = if y[5] < self.t_max {
            dy[5]
        } else {
            self.t_max - y[5]
        };

        // calculate fugacity components
        let mut fug = State::default();
        fug = fug.add_scalar(1.); // vector containing ones

        // helper variables:
        let (t, p) = (temp, self.pressure);
        let t_squared = t.powi(2);
        let p_squared = p.powi(2);

        // fugacity calculations as described in model:
        fug[0] = 0.93431737 + 0.3101804 * 10f64.powi(-3) * t + 0.295896 * 10f64.powi(-3) * p
            - 0.2707279 * 10f64.powi(-6) * t_squared
            + 0.4775207 * 10f64.powi(-6) * p_squared;

        fug[1] = E.powf(
            E.powf(-3.84027 * t.powf(1.25) + 0.541) * p
                - E.powf(-0.012637 * t.powf(0.5) - 15.980) * p.powf(2.0)
                + 300.0 * E.powf(-0.0119017 * t - 5.941) * (E.powf(-p / 300.0) - 1.0),
        );

        fug[2] = 0.1438996 + 0.2028538 * 10f64.powi(-2) * t
            - 0.448762 * 10f64.powi(-3) * p
            - 0.1142945 * 10f64.powi(-5) * t_squared
            + 0.2761216 * 10f64.powi(-6) * p_squared;

        // component-wise multiplication
        *dy = dy.component_mul(&fug);
    }

    fn solout(&mut self, _x: f64, _y: &State, dy: &State) -> bool {
        // stop solver if changes of ammonia are close to zero.
        return dy[2] < AMMONIA_THRESHOLD;
    }
}