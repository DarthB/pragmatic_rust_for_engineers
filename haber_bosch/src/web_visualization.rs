use plotters::{prelude::*, style::full_palette::GREY_100};
use plotters_canvas::CanvasBackend;
use std::ops::Range;

use crate::configuration::HaberBoschInstance;

use crate::{console_log, visualization::*};

fn comb_ranges(r1: Range<f32>, r2: Range<f32>) -> Range<f32> {
    r1.start.min(r2.start)..r1.end.max(r2.end)
}

pub fn draw_concentrations_for_canvas(
    canvas_id: &str,
    inst: &HaberBoschInstance,
    inst2: Option<HaberBoschInstance>,
    ranges: Option<(Range<f32>, Range<f32>, Range<f32>)>,
) -> Result<
    (
        impl Fn((i32, i32)) -> Option<(f32, f32)>,
        impl Fn((i32, i32)) -> Option<(f32, f32)>,
    ),
    Box<dyn std::error::Error>,
> {
    let draw_area = CanvasBackend::new(canvas_id)
        .expect("cannot find canvas")
        .into_drawing_area();
    draw_area.fill(&WHITE)?;

    let (xrange, crange, trange) = if ranges.is_some() {
        ranges.unwrap()
    } else {
        if let Some(alt) = &inst2 {
            (
                comb_ranges(inst.get_x_range(), alt.get_x_range()),
                comb_ranges(
                    inst.get_concentration_range(),
                    alt.get_concentration_range(),
                ),
                comb_ranges(inst.get_temperature_range(), alt.get_temperature_range()),
            )
        } else {
            (
                inst.get_x_range(),
                inst.get_concentration_range(),
                inst.get_temperature_range(),
            )
        }
    };

    let mut chart = prepare_dual_chart(
        &draw_area,
        format!("Haber-Bosch Concentration Balances over Length",).as_ref(),
        ("Length Indicator", "Concentration as partial Fractions"),
        xrange,
        crange,
        trange,
        "Temperature [°C]",
    );

    const CVAL: u8 = 64;
    let colors = [
        RED,
        BLUE,
        GREEN,
        RGBColor(CVAL, 0, 0),
        RGBColor(0, 0, CVAL),
        RGBColor(0, CVAL, 0),
    ];
    let labels = [
        "Nitrogen",
        "Hydrogen",
        "Ammonia",
        "Nitrogen (alt)",
        "Hydrogen(alt)",
        "Ammonia (alt)",
    ];

    let instances = if let Some(alt) = inst2 {
        vec![inst.clone(), alt]
    } else {
        vec![inst.clone()]
    };

    console_log!("Num Models for Drawing: {}!", instances.len());
    let mut offset = 0;
    for cur in instances {
        for idx in 0..=2 {
            let style = ShapeStyle {
                color: colors[offset + idx].to_rgba(),
                filled: false,
                stroke_width: 1,
            };

            let temp = chart.draw_series(LineSeries::new(
                cur.iter_my(idx, true),
                &colors[offset + idx],
            ))?;
            if offset < 3 {
                temp.label(labels[offset + idx]).legend(move |(x, y)| {
                    PathElement::new(vec![(x, y), (x + 20, y)], style.to_owned())
                });
            }
        }

        let tcol = if offset > 0 {
            RGBColor(128, 128, 128)
        } else {
            RGBColor(0, 0, 0)
        };

        let label = if offset > 0 {
            "Temperature [C°] (alt)"
        } else {
            "Temperature [C°]"
        };

        let style = ShapeStyle {
            color: tcol.to_rgba(),
            filled: false,
            stroke_width: 1,
        };

        let temp = chart.draw_secondary_series(LineSeries::new(
            cur.iter_my(5, false).map(|(x, t)| (x, t)),
            &tcol,
        ))?;
        if offset < 3 {
            temp.label(label).legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], style.to_owned())
            });
        }
        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .background_style(&GREY_100)
            .position(SeriesLabelPosition::UpperRight)
            .label_font(("sans-serif", 24).into_font())
            .draw()?;

        offset = offset + 3;
    }

    Ok(chart.into_coord_trans_pair())
}

pub fn draw_temperature_over_yield_for_canvas(
    canvas_id: &str,
    conf: &HaberBoschInstance,
    conf2: Option<HaberBoschInstance>,
    ranges: Option<(Range<f32>, Range<f32>, Range<f32>)>,
) -> Result<(impl Fn((i32, i32)) -> Option<(f32, f32)>, i32), Box<dyn std::error::Error>> {
    let draw_area = CanvasBackend::new(canvas_id)
        .expect("Canvas nto found")
        .into_drawing_area();
    draw_area.fill(&WHITE)?;

    let mut temp_range = if let Some(alt) = &conf2 {
        comb_ranges(conf.get_temperature_range(), alt.get_temperature_range())
    } else {
        conf.get_temperature_range()
    };

    let mut max_ammonia = conf
        .iter_my(2, true)
        .map(|(_, a)| a)
        .max_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap())
        .unwrap();
    max_ammonia = if let Some(alt) = &conf2 {
        alt.iter_my(2, true)
            .map(|(_, a)| a)
            .max_by(|lhs, rhs| lhs.partial_cmp(rhs).unwrap())
            .unwrap()
    } else {
        max_ammonia
    };

    if let Some((_xr, cr, tr)) = ranges {
        temp_range = tr;
        max_ammonia = cr.end;
    }

    // Function from homework (refactoring prepare_chart function in Part 1)
    let mut chart = prepare_chart(
        &draw_area,
        format!("Haber-Bosch Temperature over Ammonia Yield",).as_str(),
        ("Concentration as Partial Fraction", "Temperature"),
        0f32..max_ammonia,
        temp_range,
        true,
    );

    let it_ammonia = conf.iter_my(2, true).map(|pair| pair.1);
    let it_temperature = conf.iter_my(5, false).map(|(_, t)| t);

    chart
        .draw_series(LineSeries::new(it_ammonia.zip(it_temperature), &BLACK))?
        .label("Zig Zag")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    if let Some(alt) = &conf2 {
        let it_ammonia = alt.iter_my(2, true).map(|pair| pair.1);
        let it_temperature = alt.iter_my(5, false).map(|(_, t)| t);

        chart
            .draw_series(LineSeries::new(
                it_ammonia.zip(it_temperature),
                RGBColor(128, 128, 128),
            ))?
            .label("Zig Zag")
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));
    }

    draw_area.present()?;

    Ok((chart.into_coord_trans(), 1))
}