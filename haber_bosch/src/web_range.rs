use std::str::FromStr;

use wasm_bindgen::prelude::*;

use crate::configuration::Catalyst;
use crate::v2_hints::*; // constants

//-------------------------------------------------------------------------------------------------
// NEXT Implementatio of WebRange for restricting formular elements
//-------------------------------------------------------------------------------------------------

/// A communication type, WASM cannot handle tuples or the RangeInclusive type of Rust yet,
/// for this we implemented that helper type. We use the f64 to store integers too. Which is
/// fine as we can store integers up to 2^52 with it.
#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
pub struct WebRange {
    pub min_val: usize,
    pub max_val: usize,
    pub step: usize,
    pub def_val: usize,

    pub factor: f64,
}

impl WebRange {
    pub fn clone_with_def(&self, def: usize) -> Self {
        let mut reval = self.clone();
        reval.def_val = def;
        reval
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct WebAxisRange {
    pub length_axis_range: WebRange,
    pub concentration_axis_range: WebRange,
    pub temp_min_range: WebRange,
    pub temp_max_range: WebRange,
}

impl Default for WebAxisRange {
    fn default() -> Self {
        WebAxisRange {
            length_axis_range: WebRange {
                min_val: 1,
                max_val: 11,
                step: 1,
                def_val: 8,
                factor: 0.1,
            },
            concentration_axis_range: WebRange {
                min_val: 1,
                max_val: 11,
                step: 1,
                def_val: 8,
                factor: 0.1,
            },
            temp_min_range: WebRange {
                min_val: 330,
                max_val: 430,
                step: 5,
                def_val: 400,
                factor: 1.,
            },
            temp_max_range: WebRange {
                min_val: 400,
                max_val: 500,
                step: 5,
                def_val: 450,
                factor: 1.,
            },
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, Clone)]
pub struct WebModelRange {
    pub pressure_range: WebRange,
    pub num_beds_range: WebRange,
    pub bed_start_temp_ranges: Vec<WebRange>,

    pub axis_config: WebAxisRange,
}

#[wasm_bindgen]
impl WebModelRange {
    pub fn from_catalyst(cat: &str) -> Self {
        let catalyst = Catalyst::from_str(cat).unwrap_or(Catalyst::KMIR);

        let (pressure, st_tmpl) = match catalyst {
            Catalyst::KMIR => (
                WebRange {
                    min_val: 180,
                    max_val: 220,
                    step: 1,
                    def_val: 200,
                    factor: 1.,
                },
                WebRange {
                    min_val: 350,
                    max_val: 470,
                    step: 1,
                    def_val: 0,
                    factor: 1.,
                },
            ),
            Catalyst::FN => (
                WebRange {
                    min_val: 85,
                    max_val: 115,
                    step: 1,
                    def_val: 100,
                    factor: 1.,
                },
                WebRange {
                    min_val: 300,
                    max_val: 420,
                    step: 5,
                    def_val: 0,
                    factor: 1.,
                },
            ),
        };

        let start_temps = match catalyst {
            Catalyst::FN => vec![
                st_tmpl.clone_with_def(FN_B1_TS as usize - 273),
                st_tmpl.clone_with_def(FN_B2_TS as usize - 273),
                st_tmpl.clone_with_def(FN_B3_TS as usize - 273),
            ],
            Catalyst::KMIR => vec![
                st_tmpl.clone_with_def(KMIR_B1_TS as usize - 273),
                st_tmpl.clone_with_def(KMIR_B2_TS as usize - 273),
                st_tmpl.clone_with_def(KMIR_B3_TS as usize - 273),
            ],
        };

        WebModelRange {
            pressure_range: pressure,
            num_beds_range: WebRange {
                min_val: 1,
                max_val: 3,
                step: 1,
                def_val: 2,
                factor: 1.,
            },
            bed_start_temp_ranges: start_temps,
            axis_config: WebAxisRange::default(),
        }
    }
}
