use std::{ops::Range, str::FromStr};

use crate::simulation::{self, State, HaberBoschModel, HaberBoschSolverInfo};

use::itertools::Itertools;

use wasm_bindgen::prelude::*;

use crate::v2_hints::*;

#[derive(Debug, Copy, Clone)]
pub enum Catalyst {
    KMIR,
    FN,
}

impl FromStr for Catalyst {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "kmir" => Ok(Catalyst::KMIR),
            "fn" => Ok(Catalyst::FN),
            _ => Err("Unknown catalyst".to_owned()),
        }
    }
}

/// This data structure answers the question:
/// // What data needs to be store and how can we support any number of reactor beds?
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, Default)]
pub struct HaberBoschBedSetup {
    pub beta: f64,
    pub t_start: f64,
    pub t_slope: f64,
    pub t_max: f64,
}

#[wasm_bindgen]
impl HaberBoschBedSetup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_constants(idx: u32, catalyst: &str) -> Self {
        let cat = Catalyst::from_str(catalyst)
            .map_err(|e| panic!("Cannot create HaberBoschBedSetup: {}", e))
            .unwrap();

        match cat {
            Catalyst::KMIR => match idx {
                0 => HaberBoschBedSetup {
                    beta: KMIR_B1_B,
                    t_start: KMIR_B1_TS,
                    t_slope: KMIR_B1_TR,
                    t_max: KMIR_B1_TM,
                },
                1 => HaberBoschBedSetup {
                    beta: KMIR_B2_B,
                    t_start: KMIR_B2_TS,
                    t_slope: KMIR_B2_TR,
                    t_max: KMIR_B2_TM,
                },
                2 => HaberBoschBedSetup {
                    beta: KMIR_B3_B,
                    t_start: KMIR_B3_TS,
                    t_slope: KMIR_B3_TR,
                    t_max: KMIR_B3_TM,
                },
                _ => panic!("BED {} not supported yet", idx),
            },
            Catalyst::FN => match idx {
                0 => HaberBoschBedSetup {
                    beta: FN_B1_B,
                    t_start: FN_B1_TS,
                    t_slope: FN_B1_TR,
                    t_max: FN_B1_TM,
                },
                1 => HaberBoschBedSetup {
                    beta: FN_B2_B,
                    t_start: FN_B2_TS,
                    t_slope: FN_B2_TR,
                    t_max: FN_B2_TM,
                },
                2 => HaberBoschBedSetup {
                    beta: FN_B3_B,
                    t_start: FN_B3_TS,
                    t_slope: FN_B3_TR,
                    t_max: FN_B3_TM,
                },
                _ => panic!("BED {} not supported yet", idx),
            },
        }
    }
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

    pub fn get_x_range(&self) -> Range<f32> {
        let last = self
            .reactor_results
            .iter()
            .map(|r| r.x_out.iter().last().unwrap())
            .rev()
            .nth(0)
            .unwrap();
        0f32..(*last as f32)
    }

    pub fn get_concentration_range(&self) -> Range<f32> {
        let cmp = |a: &f64, b: &f64| a.partial_cmp(b).unwrap();

        let max_conc = self
            .reactor_results
            .iter()
            .flat_map(|el| el.y_out.iter())
            .map(|mat| {
                mat.iter().take(5).copied().max_by(cmp).unwrap() / mat.iter().take(5).sum::<f64>()
            })
            .max_by(cmp)
            .unwrap() as f32;

        0f32..max_conc + 0.1
    }

    pub fn iter_my<'a>(
        &'a self,
        component: usize,
        normalize: bool,
    ) -> MyIterator<'a> {
        if component > 5 {
            panic!("Component too high");
        }

        MyIterator {
            normalize,
            comp_idx: component,
            bed_idx: 0,
            ele_idx: 0,
            instance: self,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MyIterator<'a> {
    normalize: bool,
    comp_idx: usize,

    bed_idx: usize,
    ele_idx: usize,

    instance: &'a HaberBoschInstance,
}

impl<'a> Iterator for MyIterator<'a> {
    type Item = (f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.bed_idx >= self.instance.reactor_results.len() {
            None
        } else {
            
            let bed_res = &self.instance.reactor_results[self.bed_idx];
            let x = bed_res.x_out[self.ele_idx] as f32;

            let comp = bed_res.y_out[self.ele_idx];
            let mut y = *comp.iter().nth(self.comp_idx).unwrap() as f32;

            if self.comp_idx == 5 {
                // temperature
                y = y - 273.;
            } else if self.normalize {
                let sum: f32 = comp
                    .iter()
                    .take(5)
                    .map(|x| *x as f32)
                    .sum();
                y = y /sum;
            }

            // point to next element
            self.ele_idx = self.ele_idx + 1;
            if self.ele_idx >= bed_res.x_out.len() {
                self.ele_idx = 0;
                self.bed_idx = self.bed_idx + 1;
            }

            Some((x, y))
        }
    }
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