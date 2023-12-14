// Solver specific
pub const AMMONIA_THRESHOLD: f64 = 0.001;

// Catalyst FN Constants
pub const FN_EA: f64 = 38007.; // Activation energy
pub const FN_BIG_A: f64 = 7.6683e+15; // Pre exponential factor
pub const FN_ALPHA: f64 = 0.4;

pub const FN_REACTOR_PRESSURE: f64 = 100.; // pressure inside the reactor for FN case study

// FN Bed 1 Constants
pub const FN_B1_TS: f64 = 370. + 273.; // [K°] Starting temperature
pub const FN_B1_TR: f64 = 30.; // [K°] Raise of temperature
pub const FN_B1_TM: f64 = 490. + 237.; // [K°] Max temperature (constants invalid out of this are)
pub const FN_B1_B: f64 = -2.691122; // beta

// FN Bed 2 Constants
pub const FN_B2_TS: f64 = 350. + 273.; // [K°] Starting temperature
pub const FN_B2_TR: f64 = 15.; // [K°] Raise of temperature
pub const FN_B2_TM: f64 = 490. + 237.; // [K°] Max temperature (constants invalid out of this are)
pub const FN_B2_B: f64 = -2.708; // beta

// Catalyst KMIR Constants
pub const KMIR_EA: f64 = 40131.; // Activation energy
pub const KMIR_BIG_A: f64 = 1.6066e+15; // Pre exponential factor
pub const KMIR_ALPHA: f64 = 0.5;

pub const KMIR_REACTOR_PRESSURE: f64 = 200.; // pressure inside the reactor for KMIR case study

// KMIR Bed 1 Constants
pub const KMIR_B1_TS: f64 = 440. + 273.; // [K°] Starting temperature
pub const KMIR_B1_TR: f64 = 10.; // [K°] Raise of temperature
pub const KMIR_B1_TM: f64 = 490. + 237.; // [K°] Max temperature (constants invalid out of this are)
pub const KMIR_B1_B: f64 = -2.691122; // beta

// KMIR Bed 2 Constants
pub const KMIR_B2_TS: f64 = 400. + 273.; // [K°] Starting temperature
pub const KMIR_B2_TR: f64 = 7.5; // [K°] Raise of temperature
pub const KMIR_B2_TM: f64 = 490. + 237.; // [K°] Max temperature (constants invalid out of this are)
pub const KMIR_B2_B: f64 = -2.708; // beta

// The function that calculates our more sophistcated model
// Feel free to copy this block instead of implementing it on your own.
// Check for the variables that need to be provided by your model, therefore see self.*, e.g. self.beta
/*
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
*/
