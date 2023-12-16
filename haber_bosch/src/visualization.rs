use std::ops::Range;

use plotters::{
    chart::DualCoordChartContext,
    coord::{types::RangedCoordf32, Shift},
    prelude::*,
};

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
        .build_cartesian_2d(raise_range(x_range, 1.05f32), raise_range(y_range, 1.05f32))
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
