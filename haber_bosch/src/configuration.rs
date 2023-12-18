use std::ops::Range;

use crate::simulation::{self, State, HaberBoschModel, HaberBoschSolverInfo};

use::itertools::Itertools;

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
#[derive(Debug, Clone)]
pub struct HaberBoschInstance { 
    partial_pressure: f64,
    catalyst: Catalyst,

    reactor_beds: Vec<HaberBoschBedSetup>,
    reactor_results: Vec<HaberBoschBedResult>,
}

// NEW CODE:
impl HaberBoschInstance {
    pub fn len(&self) -> usize {
        self.reactor_beds.len()
    }

    pub fn cat(&self) -> Catalyst {
        self.catalyst
    }

    pub fn pres(&self) -> f64 {
        self.partial_pressure
    }

    pub fn add_next_results(&mut self, x: Vec<f64>, y: Vec<simulation::State>) {
        if self.reactor_results.len() >= self.reactor_beds.len() {
            panic!("There cannot be more results then beds");
        }

        self.reactor_results.push(HaberBoschBedResult { x_out: x, y_out: y })
    }

    pub fn get_solver_info(&self, idx: usize) -> simulation::HaberBoschSolverInfo {
        if idx > self.reactor_results.len() {
            panic!("Not enough results");
        }

        let model = HaberBoschModel::new(
            self.pres(), 
            self.cat(), 
            self.reactor_beds[idx]);

        let x0 = if idx == 0 {
            0.
        } else {
           *self.reactor_results[idx-1].x_out.iter().last().unwrap()
        };

        let mut y0 = if idx == 0 {
        
            // the partial gas pressure
            let mut pp = State::new(
                0.2391,
                0.623,
                0.0413,
                0.0793,
                0.0172,
                0.,
            );
            pp = pp * self.pres();
            pp
        } else {
            *self.reactor_results[idx-1].y_out.iter().last().unwrap()
        };
        y0[5] = self.reactor_beds[idx].t_start;

        HaberBoschSolverInfo { model, x0, y0 }
    }

    pub fn print_summary(&self) {
        let len_iter = self.reactor_results.iter().map(|el| *el.x_out.last().unwrap());

        println!("Summary based on {}", self.catalyst.to_string());

        print!("Reactor Length: ");
        let mut before = 0.;
        for (idx, len) in len_iter.clone().enumerate() {
            if idx > 0 {
                print!(" + ");
            }
            print!("{:.3}", len - before);
            before = before + len;
        }
        println!(" = {:.3}", len_iter.last().unwrap());

        let last_y = self.reactor_results.iter().last().unwrap().y_out.last().unwrap();
        println!(
            "Final Yield: {}\n",
            last_y[2] / (last_y.iter().take(5).sum::<f64>())
        );
    }

    pub fn get_temperature_range(&self) -> Range<f32> {
        // use::itertools::Itertools;
        let min_max_res = self.reactor_results.iter()
            .flat_map(|x| x.y_out.iter())
            .map(|mat| mat[5] as f32 - 273f32)
            .minmax_by(|lhs, rhs| {
                lhs.partial_cmp(rhs).unwrap()
            });

        match min_max_res {
            itertools::MinMaxResult::NoElements => 0f32..1f32,
            itertools::MinMaxResult::OneElement(x) => x-0.5f32..x+0.5f32,
            itertools::MinMaxResult::MinMax(a, b) => a..b,
        }
    }



    pub fn iter_my<'a>(
        &'a self,
        component: usize,
        normalize: bool,
    ) -> MyIterator<'a> {
        todo!{"Build Iterator"}
    }
}

#[derive(Debug, Clone)]
pub struct MyIterator<'a> {
    instance: &'a HaberBoschInstance,
}












pub struct HaberBoschInstanceBuilder {
    wip: HaberBoschInstance,
}

impl HaberBoschInstanceBuilder {
    pub fn create(p: f64, c: Catalyst) -> Self {
        HaberBoschInstanceBuilder { 
            wip: HaberBoschInstance { 
                partial_pressure: p, 
                catalyst: c, 
                reactor_beds: vec![], 
                reactor_results: vec![] 
        } }
    }

    pub fn add_bed(mut self, t_start: f64, t_slope: f64, t_max: f64, beta: f64) -> Self {
        self.wip.reactor_beds.push(HaberBoschBedSetup { beta, t_start, t_slope, t_max });
        self
    }

    pub fn build(self) -> HaberBoschInstance {
        if self.wip.reactor_beds.is_empty() {
            panic!("Reactor bed is require");
        }

        self.wip
    }
}

impl ToString for Catalyst {
    fn to_string(&self) -> String {
        match self {
            Catalyst::FN => "Catalyst FN".to_owned(),
            Catalyst::KMIR => "Catalyst KMIR".to_owned(),
        }
    }
}