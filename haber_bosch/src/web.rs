use std::ops::Range;
use std::str::FromStr;

use wasm_bindgen::prelude::*;

use crate::{configuration, simulation, v2_hints::*, web_range, web_visualization};
use crate::{configuration::Catalyst, console_log};

//-------------------------------------------------------------------------------------------------
// Next Implementatin of WebModelInput and WebInput structures
//-------------------------------------------------------------------------------------------------

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct WebModelInput {
    catalyst: Catalyst,
    pub pressure: f64,
    pub num_beds: usize,
    pub beds: Vec<crate::configuration::HaberBoschBedSetup>,
}

#[wasm_bindgen]
impl WebModelInput {
    pub fn new_kmir() -> Self {
        WebModelInput {
            catalyst: Catalyst::KMIR,
            pressure: KMIR_REACTOR_PRESSURE,
            num_beds: 2,
            beds: vec![],
        }
    }

    pub fn new_fn() -> Self {
        WebModelInput {
            catalyst: Catalyst::FN,
            pressure: FN_REACTOR_PRESSURE,
            num_beds: 2,
            beds: vec![],
        }
    }

    pub fn set_catalyst(&mut self, catalyst: &str) {
        self.catalyst = Catalyst::from_str(catalyst).unwrap_or(Catalyst::KMIR);
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct WebInput {
    /// input for the main model of the Haber-Bosch process that will get rendered
    pub main: WebModelInput,

    /// optional secondary input of a Haber-Bosch process model that acts as a diff view
    pub alt: Option<WebModelInput>,

    /// optional settings for the axis of the plot
    pub axis_settings: Option<WebAxisInput>,
}

#[wasm_bindgen]
impl WebInput {
    pub fn new() -> WebInput {
        WebInput::default()
    }
}

impl Default for WebInput {
    fn default() -> Self {
        Self {
            main: WebModelInput::new_kmir(),
            alt: Some(WebModelInput::new_fn()),
            axis_settings: Some(WebAxisInput::default()),
        }
    }
}

//-------------------------------------------------------------------------------------------------
// NEXT: WebChart Implementation
//-------------------------------------------------------------------------------------------------

/// Type alias for the result of a drawing function.
pub type DrawResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Type used on the JS side to convert screen coordinates to chart
/// coordinates.
#[wasm_bindgen]
pub struct WebChart {
    convert: Box<dyn Fn((i32, i32)) -> Option<(f64, f64)>>,
}

/// Result of screen to chart coordinates conversion.
#[wasm_bindgen]
pub struct WebPoint {
    pub x: f64,
    pub y: f64,
}

#[wasm_bindgen]
impl WebChart {
    pub fn draw_concentration_balances(
        canvas_id: &str,
        input: WebInput,
    ) -> Result<WebChart, JsValue> {
        console_log!("{:?}!", input);
        let inst = WebChart::simulate(&input.main);

        let inst2 = if let Some(alt) = input.alt {
            console_log!("Simulate alt");
            Some(WebChart::simulate(&alt))
        } else {
            console_log!("Only one scenario");
            None
        };

        let axis_override = if let Some(axiss) = input.axis_settings {
            Some(axiss.to_range_tuple())
        } else {
            None
        };

        let map_coord =
            web_visualization::draw_concentrations_for_canvas(canvas_id, &inst, inst2, axis_override)
                .unwrap()
                .0;

        Ok(WebChart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    pub fn draw_temperature_over_yield(
        canvas_id: &str,
        input: &WebInput,
    ) -> Result<WebChart, JsValue> {
        let inst = WebChart::simulate(&input.main);

        let inst2 = if let Some(alt) = &input.alt {
            console_log!("Simulate alt");
            Some(WebChart::simulate(alt))
        } else {
            console_log!("Only one scenario");
            None
        };

        let axis_override = if let Some(axiss) = input.axis_settings {
            Some(axiss.to_range_tuple())
        } else {
            None
        };

        let map_coord = web_visualization::draw_temperature_over_yield_for_canvas(
            canvas_id,
            &inst,
            inst2,
            axis_override,
        )
        .unwrap()
        .0;

        Ok(WebChart {
            convert: Box::new(move |coord| map_coord(coord).map(|(x, y)| (x.into(), y.into()))),
        })
    }

    fn simulate(model_input: &WebModelInput) -> configuration::HaberBoschInstance {
        let mut builder = configuration::HaberBoschInstanceBuilder::create(
            model_input.pressure,
            model_input.catalyst,
        );
        for i in 0..model_input.num_beds {
            builder = builder.add_bed(
                model_input.beds[i].t_start,
                model_input.beds[i].t_slope,
                model_input.beds[i].t_max,
                model_input.beds[i].beta,
            );
        }
        let mut inst = builder.build();

        simulation::sequential_simulation(&mut inst, false);
        inst
    }

    /// This function can be used to convert screen coordinates to
    /// chart coordinates.
    pub fn coord(&self, x: i32, y: i32) -> Option<WebPoint> {
        (self.convert)((x, y)).map(|(x, y)| WebPoint { x, y })
    }
}

//-------------------------------------------------------------------------------------------------
// Next: WebAxisInput implementation
//-------------------------------------------------------------------------------------------------

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct WebAxisInput {
    pub length_max: f32,
    pub concentration_max: f32,
    pub min_temp: f32,
    pub max_temp: f32,
}

impl WebAxisInput {
    pub fn to_range_tuple(&self) -> (Range<f32>, Range<f32>, Range<f32>) {
        (
            0f32..self.length_max,
            0f32..self.concentration_max,
            self.min_temp..self.max_temp,
        )
    }
}

impl Default for WebAxisInput {
    fn default() -> Self {
        let config = web_range::WebAxisRange::default();
        Self {
            length_max: config.length_axis_range.def_val as f32,
            concentration_max: config.concentration_axis_range.def_val as f32,
            min_temp: config.temp_min_range.def_val as f32,
            max_temp: config.temp_max_range.def_val as f32,
        }
    }
}
