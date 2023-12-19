#![allow(unreachable_code)]
#![allow(unused)]
use crate::{configuration::{Catalyst, HaberBoschInstanceBuilder}, simulation::sequential_simulation, visualization::prepare_chart};

use plotters::prelude::*;

pub mod simulation;
pub mod configuration;
pub mod visualization;

pub mod v2_hints; use plotters::element::BitMapElement;
// this contains our model constants
use v2_hints::*;


// Reactor bed related constants:
// KMIR_B1_TS, KMIR_B1_TR, KMIR_B1_TM, KMIR_B1_B
// KMIR_B2_TS, KMIR_B2_TR, KMIR_B2_TM, KMIR_B2_B
// FN_B1_TS, FN_B1_TR, FN_B1_TM, FN_B1_B
// FN_B2_TS, FN_B2_TR, FN_B2_TM, FN_B2_B


fn main() -> Result<(), Box<dyn std::error::Error>>
{
    
    println!("Hello sequential Simulation!");
    // case studies with different catalysts:
    let studies = [(Catalyst::KMIR, "HB_KMIR"), (Catalyst::FN, "HB_FN")]; 
    for (cat, fn_pref) in studies
    {
        // configure Haber-Bosch case-study (mostly module configuration in configuration.rs)
        let mut conf = match cat {
            Catalyst::KMIR => {
                HaberBoschInstanceBuilder::create(KMIR_REACTOR_PRESSURE, cat)
                    .add_bed(KMIR_B1_TS, KMIR_B1_TR, KMIR_B1_TM, KMIR_B1_B)
                    .add_bed(KMIR_B2_TS, KMIR_B2_TR, KMIR_B2_TM, KMIR_B2_B)
                    .build()
            }
            Catalyst::FN => {
                HaberBoschInstanceBuilder::create(FN_REACTOR_PRESSURE, cat)
                    .add_bed(FN_B1_TS, FN_B1_TR, FN_B1_TM, FN_B1_B)
                    .add_bed(FN_B2_TS, FN_B2_TR, FN_B2_TM, FN_B2_B)
                    .build()
            },
        };

        println!("{:?}", conf);

        // simulate Haber-Bosch case-study (mostly module simulation in simulation.rs)
        sequential_simulation(&mut conf, false);
        conf.print_summary();

        // visualize Haber-Bosch case-study (feed module visualization (visualization.rs) from configuration)
        let fn_conc = fn_pref.to_owned() + "_conc.png";
        visualization::draw_concentations(&fn_conc.as_str(), &conf)?;

        let fn_temp_over_yield = fn_pref.to_owned() + "_temp_yield.png";
        visualization::draw_temperature_over_yield(fn_temp_over_yield.as_str(), &conf)?;
    }

    Ok(())
}