use std::error::Error;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn Error>>
{
    println!("Hello Plot!");
    let da = BitMapBackend::new("my.png", (800,600)).into_drawing_area();
    da.fill(&RED)?;
    match da.present() {
        Ok(_) => {},
        Err(_) => println!("Could not save file"),
    }

    Ok(())
}