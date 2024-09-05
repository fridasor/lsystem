use plotters::prelude::*;
use crate::utils::lsystem::LSystem;

pub fn lsystem_plotter(out_filename: &str, lsystem: &LSystem) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(&out_filename, (1024, 1024)).into_drawing_area();

    let ymax = 20.0;
    let xmax = ymax / 2.0;

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(lsystem.rulestring.clone(), ("monospace", 30))
        .build_cartesian_2d(-xmax..xmax, -1.0..(ymax-1.0))?;

    for v in lsystem.vertices.clone() {
        chart.draw_series(
            std::iter::once(PathElement::new(
                v, 
                ShapeStyle {
                    color: BLACK.into(), 
                    filled: false, 
                    stroke_width: 2,
                }
            ))
        )?;
    }

    root.present().expect("Could not plot vertices!");
    println!("Plot exported to {:?}", out_filename);
    Ok(())
}

pub fn main_result() {
    // let mut system = LSystem::new(4, "X => F[-X][+X]", "X", 30.0, 5.0);
    let mut system = LSystem::new(5, "X => F-[[X]+X]+F[+FX]-X", "X", 22.5, 1.3);
    // let mut system = LSystem::new(4, "F=>F+F-F-FF+F+F-F", "F+F+F+F", 90.0, 0.2);
    // let mut system = LSystem::new(4, "F=>FF+F-F+F+FF", "F+F+F+F", 90.0, 0.2);
 
    system.fill_string();
    system.fill_vertices();

    let _ = lsystem_plotter("data/test.png", &system);
}