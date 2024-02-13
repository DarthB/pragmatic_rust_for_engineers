use wasm_bindgen::prelude::*;

use crate::{
    configuration::{Catalyst, HaberBoschInstance, HaberBoschInstanceBuilder},
    console_log,
    simulation::sequential_simulation,
    v2_hints::*,
    web_visualization::draw_concentrations_for_canvas,
};

/// Result of screen to chart coordinates conversion.
#[wasm_bindgen]
pub struct WebPoint {
    pub x: f64,
    pub y: f64,
}

/// Type used on the JS side to convert screen coordinates to chart
/// coordinates.
#[wasm_bindgen]
pub struct WebChart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}

#[wasm_bindgen]
impl WebChart {
    fn simulate(pressure: f64, catalyst: Catalyst) -> HaberBoschInstance {
        let mut builder = HaberBoschInstanceBuilder::create(pressure, catalyst);
        builder = builder.add_bed(KMIR_B1_TS, KMIR_B1_TR, KMIR_B1_TM, KMIR_B1_B);
        builder = builder.add_bed(KMIR_B2_TS, KMIR_B2_TR, KMIR_B2_TM, KMIR_B2_B);
        let mut inst = builder.build();

        sequential_simulation(&mut inst, false);
        inst
    }

    pub fn draw_concentration_balances(
        canvas_id: &str,
        catalyst: &str,
        pres: f64,
    ) -> Result<WebChart, JsValue> {
        console_log!(
            "Drawing on {} with {} cat and pres={}!",
            canvas_id,
            catalyst,
            pres
        );
        let cat = if catalyst == "KMIR" {
            Catalyst::KMIR
        } else {
            Catalyst::FN
        };
        let inst = WebChart::simulate(pres, cat);

        let map_coord = draw_concentrations_for_canvas(canvas_id, &inst, None, None)
            .unwrap()
            .0;

        Ok(WebChart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    pub fn coord(&self, x: i32, y: i32) -> Option<WebPoint> {
        (self.convert)((x, y)).map(|(x, y)| WebPoint { x, y })
    }
}
