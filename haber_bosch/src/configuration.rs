use crate::simulation;

#[derive(Debug, Copy, Clone)]
pub enum Catalyst {
    KMIR,
    FN,
}

/// This data structure answers the question:
/// // What data needs to be store and how can we support any number of reactor beds?
#[derive(Debug, Copy, Clone)]
pub struct HaberBoschBedSetup {
    pub beta: f64,
    pub t_start: f64,
    pub t_slope: f64,
    pub t_max: f64,
}

/// This data structure answers the question:
/// Use this structure to store results of ODE-solver (x_out and y_out), what is the dimensionality of y_out?
#[derive(Debug, Clone)]
pub struct HaberBoschBedResult {
    pub x_out: Vec<f64>,
    pub y_out: Vec<simulation::State>, 
    // --> simulation::State is 6D vector (nitrogen, hydrogen, ammonia, argon, methan and temperature)
}

// Introduce a nested data-structure that is capable of representing a Haber-Bosch configuration in configuration.rs, think about:
// What data needs to be store and how can we support any number of reactor beds?
// Use this structure to store results of ODE-solver (x_out and y_out), what is the dimensionality of y_out?
// What data is needed by the ODE-solver and how can we provide it?

// Let's code
pub struct HaberBoschInstance { 
    partial_pressure: f64,
    catalyst: Catalyst,

    reactor_beds: Vec<HaberBoschBedSetup>,
    reactor_results: Vec<HaberBoschBedResult>,
}

