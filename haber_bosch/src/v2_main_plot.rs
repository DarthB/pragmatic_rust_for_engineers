use std::error::Error;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn Error>>
{
    println!("Hello Plot!");
    let da = BitMapBackend::new("my.png", (800,600)).into_drawing_area();
    da.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&da)
        .margin(12)
        .set_left_and_bottom_label_area_size(40)
        .build_cartesian_2d(-3f32..3f32, -30f32..30f32)?;

    chart.configure_mesh().x_desc("outside data").y_desc("magic numbers").draw()?;

    let coords = (-300..300).into_iter()
        .map(|x| x as f32 / 100.)
        .map(|x| (x, x*x*x));

    let coords: Vec<(f32, f32)> = coords.collect();
    chart.draw_series(LineSeries::new(coords, &BLACK))?;

    match da.present() {
        Ok(_) => {},
        Err(_) => println!("Could not save file"),
    }

    Ok(())
}