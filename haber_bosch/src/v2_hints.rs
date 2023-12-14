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
