use std::ops::Range;

use plotters::{
    chart::DualCoordChartContext,
    coord::{types::RangedCoordf32, Shift},
    prelude::*, style::full_palette::GREY_100,
};

use crate::configuration::HaberBoschInstance;


/// Place new functions here:
#[cfg(not(target_arch="wasm32"))]
pub fn draw_temperature_over_yield(filename: &str, conf: &HaberBoschInstance) -> Result<(), Box<dyn std::error::Error>> {
    let resolution = (1920, 1080); 
    let mut draw_area = BitMapBackend::new(
        filename, 
        resolution)
        .into_drawing_area();
    draw_area.fill(&WHITE);

    let max_ammonia = conf.iter_my(2, true)
        .map(|(x, a)| a)
        .max_by(|lhs, rhs| {
            lhs.partial_cmp(rhs).unwrap()
         }).unwrap();   

    // Function from homework (refactoring prepare_chart function in Part 1)
    let mut chart = prepare_chart(&draw_area, 
        format!("Haber-Bosch Temperature over Ammonia Yield with {}", conf.cat().to_string()).as_str(), 
        ("concentration as Partial Fraction", "Temperature"),
        0f32..max_ammonia, conf.get_temperature_range(), true);

    let it_ammonia = conf.iter_my(2, true)
        .map(|pair| pair.1);
    let it_temperature = conf.iter_my(5, false)
        .map(|(x, t)| t);

    chart
        .draw_series(LineSeries::new(it_ammonia.zip(it_temperature), &BLACK))?
        .label("Zig Zag")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    draw_area.present()?;

    Ok({})
}

#[cfg(not(target_arch="wasm32"))]
pub fn draw_concentations(
    filename: &str,
    inst: &HaberBoschInstance,
) -> Result<(), Box<dyn std::error::Error>> {
    // @todo refactor into functions for part 1 plotting
    let resolution = (1920, 1080);

    let draw_area = BitMapBackend::new(filename, resolution).into_drawing_area();
    draw_area.fill(&WHITE)?;

    let mut chart = prepare_dual_chart(
        &draw_area,
        format!(
            "Haber-Bosch Concentration Balances over Length with {}",
            inst.cat().to_string()
        )
        .as_ref(),
        ("Length Indicator", "Concentration as partial Fractions"),
        inst.get_x_range(),
        inst.get_concentration_range(),
        inst.get_temperature_range(),
        "Temperature [°C]",
    );

    let colors = [RED, BLUE, GREEN];
    let labels = ["Nitrogen", "Hydrogen", "Ammonia"];

    for idx in 0..=2 {
        let style = ShapeStyle {
            color: colors[idx].to_rgba(),
            filled: false,
            stroke_width: 1,
        };

        chart
            .draw_series(LineSeries::new(inst.iter_my(idx, true), &colors[idx]))?
            .label(labels[idx])
            .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style.to_owned()));
    }

    let style = ShapeStyle {
        color: BLACK.to_rgba(),
        filled: false,
        stroke_width: 1,
    };

    chart
        .draw_secondary_series(LineSeries::new(
            inst.iter_my(5, false).map(|(x, t)| (x, t)),
            &BLACK,
        ))?
        .label("Temperature [C°]")
        .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style.to_owned()));

    chart
        .configure_series_labels()
        .border_style(&BLACK)
        .background_style(&GREY_100)
        .position(SeriesLabelPosition::UpperRight)
        .label_font(("sans-serif", 24).into_font())
        .draw()?;

    Ok(())
}


fn raise_range(orig: Range<f32>, percent: f32) -> Range<f32> {
    let range = (orig.end - orig.start) / 2.;
    let mid = orig.start + range;
    (mid - range * percent)..(mid + range * percent)
}

pub fn prepare_chart<'a, DB: DrawingBackend>(
    draw_area: &'a DrawingArea<DB, Shift>,
    caption: &str,
    label_desc: (&str, &str),
    x_range: Range<f32>,
    y_range: Range<f32>,
    with_mesh: bool,
) -> ChartContext<'a, DB, Cartesian2d<RangedCoordf32, RangedCoordf32>> {
    let font_caption: FontDesc<'_> = ("sans-serif", 32).into_font();

    let mut chart = ChartBuilder::on(draw_area)
        .caption(caption, font_caption)
        .set_left_and_bottom_label_area_size(40)
        .set_label_area_size(LabelAreaPosition::Right, 40)
        .margin(12)
        .build_cartesian_2d(
            raise_range(x_range, 1.05f32),
            raise_range(y_range, 1.05f32))
            .unwrap();

    if with_mesh {
        chart
            .configure_mesh()
            .x_desc(label_desc.0)
            .y_desc(label_desc.1)
            .draw()
            .unwrap();
    }
    chart
}

/*
// IDEA for not exposing life times: USE Closures, e.g.:
// fn build_chart(&'a self, build_fn: Fn, adapt_chart_fn: Fn) -> Result<ChartContext<'a, ...>, Error>
draw_area.build_chart(|builder| {
        builder
            .caption("Title")
            .set_left_and_bottom_label_area_size(40)
            .build_cartesian_2d(0f32..10f32, 0f32..100f32)?;
    }, |chart| {
        chart.configure.mesh()
            .x_desc("Magic Numbers")
            .y_desc("Magic Heights")
            .draw()?;
    })
*/

pub fn prepare_dual_chart<'a, DB: DrawingBackend>(
    draw_area: &'a DrawingArea<DB, Shift>,
    caption: &str,
    label_desc: (&str, &str),
    x_range: Range<f32>,
    y_range: Range<f32>,
    y2_range: Range<f32>,
    ylabel2: &str,
) -> DualCoordChartContext<
    'a,
    DB,
    Cartesian2d<RangedCoordf32, RangedCoordf32>,
    Cartesian2d<RangedCoordf32, RangedCoordf32>,
> {
    let mut chart = prepare_chart(
        draw_area,
        caption,
        label_desc,
        x_range.clone(),
        y_range,
        false,
    )
    .set_secondary_coord(
        raise_range(x_range, 1.05f32),
        raise_range(y2_range, 1.05f32),
    );

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_desc(label_desc.0)
        .y_desc(label_desc.1)
        .draw()
        .unwrap();

    chart
        .configure_secondary_axes()
        .y_desc(ylabel2)
        .draw()
        .unwrap();

    chart
}
