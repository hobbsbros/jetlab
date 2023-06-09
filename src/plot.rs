//! Plots data generated by the Jetlab calculator.

use plotters::prelude::*;

/// Plots a function provided.
pub fn plot<'a, F>(
    function: F,
    left: f64,
    right: f64,
    n: usize,
    independent: &'a str,
    dependent: &'a str,
    filename: &'a str,
) where F: Fn(f64) -> f64 {
    let title = &format!("{} vs. {}", dependent, independent);

    let drawing_area = BitMapBackend::new(filename, (1280, 1024))
        .into_drawing_area();

    drawing_area.fill(&WHITE).unwrap();

    let xl = left - (right - left)*0.2;
    let xu = right + (right - left)*0.2;

    let mut series: Vec<(f64, f64)> = Vec::new();
    let mut i = left;
    let mut min = function(i);
    let mut max = function(i);
    let di = (right - left)/(n as f64);
    while i < right {
        let f = function(i);
        series.push((i, f));
        if f > max {
            max = f;
        } else if f < min {
            min = f;
        }
        i += di;
    }

    let yl = min - (max - min)*0.2;
    let yu = max + (max - min)*0.2;
    
    let mut chart = ChartBuilder::on(&drawing_area)
        .caption(title, ("serif", 48))
        .set_label_area_size(LabelAreaPosition::Left, 80)
        .set_label_area_size(LabelAreaPosition::Bottom, 80)
        .build_cartesian_2d(xl..xu, yl..yu)
        .unwrap();
    
    chart.draw_series(
        LineSeries::new(series, &BLUE)
    ).unwrap().label(title);

    chart
        .configure_mesh()
        .y_desc(dependent)
        .x_desc(independent)
        .axis_desc_style(("serif", 30))
        .draw()
        .unwrap();
}