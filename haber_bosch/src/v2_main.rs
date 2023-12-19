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

        //------------------------------------
        // Prepared Plotting Code as in part 1
        //------------------------------------

        // visualize Haber-Bosch case-study (feed module visualization (visualization.rs) from configuration)
        let fn_temp_over_yield = fn_pref.to_owned() + "_temp_yield.png";
        let resolution = (1920, 1080); 
        let mut draw_area = BitMapBackend::new(
            fn_temp_over_yield.as_str(), 
            resolution)
            .into_drawing_area();
        draw_area.fill(&WHITE);

        // Function from homework (refactoring prepare_chart function in Part 1)
        let mut chart = prepare_chart(&draw_area, 
            format!("Haber-Bosch Temperature over Ammonia Yield with {}", cat.to_string()).as_str(), 
            ("concentration as Partial Fraction", "Temperature"),
            0f32..1f32, conf.get_temperature_range(), true);

        let it_ammonia = conf.iter_my(2, true)
            .map(|pair| pair.1);
        let it_temperature = conf.iter_my(5, false)
            .map(|(x, t)| t);

    
        let v: Vec<f32> = it_temperature.clone().collect();
        println!("{:?}", v);
        let v: Vec<f32> = it_ammonia.clone().collect();
        println!("{:?}", v);

        chart
            .draw_series(LineSeries::new(it_ammonia.zip(it_temperature), &BLACK))?
            .label("Zig Zag")
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

        draw_area.present()?;
    }

    Ok(())
}