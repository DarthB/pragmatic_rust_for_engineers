use ode_solvers::Vector6;

use crate::v2_hints::*;
use crate::configuration::{Catalyst, HaberBoschBedSetup};

pub type State = Vector6<f64>; // 5 mixture components and temperature [nitrogn, hydrogen, ammonia, argon, metan, temperature]

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