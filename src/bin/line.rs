use plotters::coord::Shift;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use rand::prelude::*;
use std::error::Error;
use std::f64::consts::PI;
use std::fs;

const FIG_300DPI: (u32, u32) = (2400, 1600); // ~8x5.33 inches @ 300 DPI
const BIG_GRID_300DPI: (u32, u32) = (3600, 2400); // larger canvas for multi-panel figures

fn linspace(start: f64, end: f64, n: usize) -> Vec<f64> {
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![start];
    }
    let step = (end - start) / ((n - 1) as f64);
    (0..n).map(|i| start + (i as f64) * step).collect()
}

fn ensure_output_dir() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("output")?;
    Ok(())
}

fn with_png_root(path: &str, size: (u32, u32)) -> Result<DrawingArea<BitMapBackend<'_>, Shift>, Box<dyn Error>> {
    let root = BitMapBackend::new(path, size).into_drawing_area();
    root.fill(&WHITE)?;
    Ok(root)
}

fn draw_mesh_f64(
    chart: &mut ChartContext<'_, BitMapBackend<'_>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    x_desc: &str,
    y_desc: &str,
) -> Result<(), Box<dyn Error>> {
    chart
        .configure_mesh()
        .x_desc(x_desc)
        .y_desc(y_desc)
        .label_style(("sans-serif", 24))
        .axis_desc_style(("sans-serif", 26))
        .draw()?;
    Ok(())
}

// 1) Multiple line plots on same axes
fn example_1() -> Result<(), Box<dyn Error>> {
    let root = with_png_root("output/line_1_multiple.png", FIG_300DPI)?;
    let x = linspace(0.0, 2.0 * PI, 200);
    let y = x.iter().map(|&v| v.sin()).collect::<Vec<_>>();

    let y_neg = y.iter().map(|&v| -v).collect::<Vec<_>>();
    let y_lin = x.iter().map(|&v| v / PI - 1.0).collect::<Vec<_>>();

    let y_k = vec![1.0, 0.7, 0.4, 0.0, -0.4, -0.7, -1.0];
    let x_k = (0..y_k.len()).map(|i| i as f64).collect::<Vec<_>>();

    let mut chart = ChartBuilder::on(&root)
        .caption("Multiple line plots", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(70)
        .build_cartesian_2d(0.0f64..(2.0 * PI), -1.5f64..1.5f64)?;

    draw_mesh_f64(&mut chart, "x", "y")?;

    // sin(x) + markers
    chart.draw_series(LineSeries::new(
        x.iter().zip(y.iter()).map(|(&a, &b)| (a, b)),
        &BLUE,
    ))?;
    chart.draw_series(
        x.iter()
            .zip(y.iter())
            .enumerate()
            .filter(|(i, _)| i % 10 == 0)
            .map(|(_, (&a, &b))| Circle::new((a, b), 5, BLUE.filled())),
    )?;

    // -sin(x) (different color)
    chart.draw_series(LineSeries::new(
        x.iter().zip(y_neg.iter()).map(|(&a, &b)| (a, b)),
        &RED,
    ))?;

    // linear transform
    chart.draw_series(LineSeries::new(
        x.iter().zip(y_lin.iter()).map(|(&a, &b)| (a, b)),
        &GREEN,
    ))?;

    // short manual series (black)
    chart.draw_series(LineSeries::new(
        x_k.iter().zip(y_k.iter()).map(|(&a, &b)| (a, b)),
        &BLACK,
    ))?;

    root.present()?;
    Ok(())
}

// 2) Plot from collection of vectors
fn example_2() -> Result<(), Box<dyn Error>> {
    let root = with_png_root("output/line_2_vectors.png", FIG_300DPI)?;

    let ys: Vec<Vec<f64>> = vec![
        vec![16.0, 5.0, 9.0, 4.0],
        vec![2.0, 11.0, 7.0, 14.0],
        vec![3.0, 10.0, 6.0, 15.0],
        vec![13.0, 8.0, 12.0, 1.0],
    ];

    let mut chart = ChartBuilder::on(&root)
        .caption("Multiple line plots (collection of vectors)", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(70)
        .build_cartesian_2d(0.0f64..3.0f64, 0.0f64..18.0f64)?;

    draw_mesh_f64(&mut chart, "x", "y")?;

    for (idx, yv) in ys.iter().enumerate() {
        let x = (0..yv.len()).map(|i| i as f64).collect::<Vec<_>>();
        let style = Palette99::pick(idx).stroke_width(3);
        chart.draw_series(LineSeries::new(
            x.iter().zip(yv.iter()).map(|(&a, &b)| (a, b)),
            style,
        ))?;
    }

    root.present()?;
    Ok(())
}

// 3) Sin function line plots
fn example_3() -> Result<(), Box<dyn Error>> {
    let root = with_png_root("output/line_3_sin_family.png", FIG_300DPI)?;
    let x = linspace(0.0, 2.0 * PI, 300);

    let y1 = x.iter().map(|&v| v.sin()).collect::<Vec<_>>();
    let y2 = x.iter().map(|&v| (v - 0.25).sin()).collect::<Vec<_>>();
    let y3 = x.iter().map(|&v| (v - 0.5).sin()).collect::<Vec<_>>();

    let mut chart = ChartBuilder::on(&root)
        .caption("Sin() function line plots", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(70)
        .build_cartesian_2d(0.0f64..(2.0 * PI), -1.3f64..1.3f64)?;

    draw_mesh_f64(&mut chart, "x", "y")?;

    chart.draw_series(LineSeries::new(x.iter().zip(y1.iter()).map(|(&a, &b)| (a, b)), &BLUE))?;
    chart.draw_series(LineSeries::new(x.iter().zip(y2.iter()).map(|(&a, &b)| (a, b)), &RED))?;
    chart.draw_series(LineSeries::new(x.iter().zip(y3.iter()).map(|(&a, &b)| (a, b)), &GREEN))?;

    root.present()?;
    Ok(())
}

// 4) Sin function line plots with markers
fn example_4() -> Result<(), Box<dyn Error>> {
    let root = with_png_root("output/line_4_sin_markers.png", FIG_300DPI)?;
    let x = linspace(0.0, 2.0 * PI, 220);

    let y1 = x.iter().map(|&v| v.sin()).collect::<Vec<_>>();
    let y2 = x.iter().map(|&v| (v - 0.25).sin()).collect::<Vec<_>>();
    let y3 = x.iter().map(|&v| (v - 0.5).sin()).collect::<Vec<_>>();

    let mut chart = ChartBuilder::on(&root)
        .caption("Sin() function line plots with markers", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(70)
        .build_cartesian_2d(0.0f64..(2.0 * PI), -1.3f64..1.3f64)?;

    draw_mesh_f64(&mut chart, "x", "y")?;

    chart.draw_series(LineSeries::new(x.iter().zip(y1.iter()).map(|(&a, &b)| (a, b)), &GREEN))?;
    chart.draw_series(LineSeries::new(x.iter().zip(y2.iter()).map(|(&a, &b)| (a, b)), &BLUE))?;
    chart.draw_series(LineSeries::new(x.iter().zip(y3.iter()).map(|(&a, &b)| (a, b)), &CYAN))?;

    // Markers on each series
    chart.draw_series(
        x.iter()
            .zip(y1.iter())
            .enumerate()
            .filter(|(i, _)| i % 12 == 0)
            .map(|(_, (&a, &b))| Circle::new((a, b), 5, GREEN.filled())),
    )?;
    chart.draw_series(
        x.iter()
            .zip(y2.iter())
            .enumerate()
            .filter(|(i, _)| i % 12 == 0)
            .map(|(_, (&a, &b))| TriangleMarker::new((a, b), 6, BLUE.filled())),
    )?;
    chart.draw_series(
        x.iter()
            .zip(y3.iter())
            .enumerate()
            .filter(|(i, _)| i % 12 == 0)
            .map(|(_, (&a, &b))| Cross::new((a, b), 6, CYAN.filled())),
    )?;

    root.present()?;
    Ok(())
}

// 5) Simple 2x1 layout (separate figure)
fn example_5() -> Result<(), Box<dyn Error>> {
    let root = with_png_root("output/line_5_tiled.png", (2400, 2200))?;
    let areas = root.split_evenly((2, 1));

    let x = linspace(0.0, 3.0, 250);
    let y1 = x.iter().map(|&v| (5.0 * v).sin()).collect::<Vec<_>>();
    let y2 = x.iter().map(|&v| (15.0 * v).sin()).collect::<Vec<_>>();

    // Top
    {
        let mut chart = ChartBuilder::on(&areas[0])
            .caption("Top Plot", ("sans-serif", 34))
            .margin(15)
            .x_label_area_size(45)
            .y_label_area_size(60)
            .build_cartesian_2d(0.0f64..3.0f64, -1.3f64..1.3f64)?;
        chart
            .configure_mesh()
            .x_desc("x")
            .y_desc("sin(5x)")
            .label_style(("sans-serif", 22))
            .axis_desc_style(("sans-serif", 24))
            .draw()?;
        chart.draw_series(LineSeries::new(
            x.iter().zip(y1.iter()).map(|(&a, &b)| (a, b)),
            &BLUE,
        ))?;
    }

    // Bottom
    {
        let mut chart = ChartBuilder::on(&areas[1])
            .caption("Bottom Plot", ("sans-serif", 34))
            .margin(15)
            .x_label_area_size(45)
            .y_label_area_size(60)
            .build_cartesian_2d(0.0f64..3.0f64, -1.3f64..1.3f64)?;
        chart
            .configure_mesh()
            .x_desc("x")
            .y_desc("sin(15x)")
            .label_style(("sans-serif", 22))
            .axis_desc_style(("sans-serif", 24))
            .draw()?;
        chart.draw_series(LineSeries::new(
            x.iter().zip(y2.iter()).map(|(&a, &b)| (a, b)),
            &RED,
        ))?;
    }

    root.present()?;
    Ok(())
}

// 6) 3x2 subplots in a single figure (6 subplots total)
fn example_6() -> Result<(), Box<dyn Error>> {
    let root = with_png_root("output/line_6_grid_3x2.png", BIG_GRID_300DPI)?;
    let areas = root.split_evenly((3, 2));

    // (0,0): sin(x) with marker indices
    {
        let x = linspace(0.0, 10.0, 100);
        let y = x.iter().map(|&v| v.sin()).collect::<Vec<_>>();
        let mut chart = ChartBuilder::on(&areas[0])
            .caption("sin(x) with marker indices", ("sans-serif", 26))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0.0f64..10.0f64, -1.3f64..1.3f64)?;
        draw_mesh_f64(&mut chart, "x", "sin(x)")?;

        chart.draw_series(LineSeries::new(
            x.iter().zip(y.iter()).map(|(&a, &b)| (a, b)),
            &BLUE,
        ))?;

        let marker_indices: Vec<usize> = (0..100).step_by(5).collect();
        chart.draw_series(marker_indices.into_iter().map(|i| {
            Circle::new((x[i], y[i]), 5, BLUE.filled())
        }))?;
    }

    // (0,1): tan(sin(x)) - sin(tan(x))
    {
        let x = linspace(-PI, PI, 20);
        let y = x
            .iter()
            .map(|&v| (v.sin()).tan() - (v.tan()).sin())
            .collect::<Vec<_>>();

        let mut chart = ChartBuilder::on(&areas[1])
            .caption("tan(sin(x)) - sin(tan(x))", ("sans-serif", 26))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(-PI..PI, -5.0f64..5.0f64)?;
        draw_mesh_f64(&mut chart, "x", "y")?;

        chart.draw_series(LineSeries::new(
            x.iter().zip(y.iter()).map(|(&a, &b)| (a, b)),
            GREEN.stroke_width(3),
        ))?;
        chart.draw_series(
            x.iter()
                .zip(y.iter())
                .map(|(&a, &b)| Circle::new((a, b), 3, BLACK.filled())),
        )?;
    }

    // (1,0): cos(5x)
    {
        let x = linspace(0.0, 10.0, 150);
        let y = x.iter().map(|&v| (5.0 * v).cos()).collect::<Vec<_>>();

        let mut chart = ChartBuilder::on(&areas[2])
            .caption("2-D Line Plot", ("sans-serif", 26))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0.0f64..10.0f64, -1.3f64..1.3f64)?;
        draw_mesh_f64(&mut chart, "x", "cos(5x)")?;
        chart.draw_series(LineSeries::new(
            x.iter().zip(y.iter()).map(|(&a, &b)| (a, b)),
            CYAN.stroke_width(3),
        ))?;
    }

    // (1,1): time plot with custom tick labels
    {
        let x: Vec<i32> = vec![0, 30, 60, 90, 120, 150, 180];
        let y: Vec<f64> = vec![0.8, 0.9, 0.1, 0.9, 0.6, 0.1, 0.3];

        let mut chart = ChartBuilder::on(&areas[3])
            .caption("Time Plot", ("sans-serif", 26))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0i32..180i32, 0.0f64..1.0f64)?;

        chart
            .configure_mesh()
            .x_desc("Time")
            .y_desc("Value")
            .x_labels(7)
            .x_label_formatter(&|v| match *v {
                0 => "00:00s".to_string(),
                30 => "30:00".to_string(),
                60 => "01:00".to_string(),
                90 => "01:30".to_string(),
                120 => "02:00".to_string(),
                150 => "02:30".to_string(),
                180 => "03:00".to_string(),
                _ => format!("{v}"),
            })
            .label_style(("sans-serif", 20))
            .axis_desc_style(("sans-serif", 24))
            .draw()?;

        chart.draw_series(LineSeries::new(
            x.iter().zip(y.iter()).map(|(&a, &b)| (a, b)),
            BLUE.stroke_width(3),
        ))?;
        chart.draw_series(x.iter().zip(y.iter()).map(|(&a, &b)| Circle::new((a, b), 5, RED.filled())))?;
    }

    // (2,0): sin(5x)
    {
        let x = linspace(0.0, 3.0, 200);
        let y = x.iter().map(|&v| (5.0 * v).sin()).collect::<Vec<_>>();

        let mut chart = ChartBuilder::on(&areas[4])
            .caption("sin(5x)", ("sans-serif", 26))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0.0f64..3.0f64, -1.3f64..1.3f64)?;
        draw_mesh_f64(&mut chart, "x", "y")?;
        chart.draw_series(LineSeries::new(
            x.iter().zip(y.iter()).map(|(&a, &b)| (a, b)),
            BLACK.stroke_width(3),
        ))?;
    }

    // (2,1): circle
    {
        let r = 2.0;
        let xc = 4.0;
        let yc = 3.0;

        let theta = linspace(0.0, 2.0 * PI, 400);
        let xs = theta.iter().map(|&t| r * t.cos() + xc).collect::<Vec<_>>();
        let ys = theta.iter().map(|&t| r * t.sin() + yc).collect::<Vec<_>>();

        // This panel is wider than it is tall; choose axis ranges that compensate to keep the circle visually round.
        let y_span = 4.0;
        let x_span = 2.0 * y_span;

        let mut chart = ChartBuilder::on(&areas[5])
            .caption("Circle", ("sans-serif", 26))
            .margin(10)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d((xc - x_span)..(xc + x_span), (yc - y_span)..(yc + y_span))?;

        draw_mesh_f64(&mut chart, "x", "y")?;
        chart.draw_series(LineSeries::new(
            xs.iter().zip(ys.iter()).map(|(&a, &b)| (a, b)),
            BLUE.stroke_width(3),
        ))?;
    }

    root.present()?;
    Ok(())
}

// --- CSV helper ---
fn download_csv_text(url: &str) -> Result<String, Box<dyn Error>> {
    let resp = reqwest::blocking::get(url)?;
    let text = resp.text()?;

    // Some CSV sources may be delivered as whitespace-separated records.
    // Normalize by converting any whitespace runs into newline separators.
    let normalized = text.split_whitespace().collect::<Vec<_>>().join("\n");
    Ok(normalized)
}

fn example_7_csv_plot() -> Result<(), Box<dyn Error>> {
    let root = with_png_root("output/line_7_csv_scatter.png", FIG_300DPI)?;

    let url = "https://raw.githubusercontent.com/mohammadijoo/Datasets/refs/heads/main/iris.csv";

    // If the CSV is on your local drive:
    // let mut rdr = csv::Reader::from_path("path/to/iris.csv")?;

    let csv_text = download_csv_text(url)?;
    let mut rdr = csv::Reader::from_reader(std::io::Cursor::new(csv_text));

    let headers = rdr.headers()?.clone();
    let idx_x = headers
        .iter()
        .position(|h| h == "sepal_length")
        .ok_or("Missing column: sepal_length")?;
    let idx_y = headers
        .iter()
        .position(|h| h == "petal_length")
        .ok_or("Missing column: petal_length")?;

    let mut xs = Vec::<f64>::new();
    let mut ys = Vec::<f64>::new();

    for rec in rdr.records() {
        let rec = rec?;
        let x: f64 = rec.get(idx_x).ok_or("Bad record")?.trim().parse()?;
        let y: f64 = rec.get(idx_y).ok_or("Bad record")?.trim().parse()?;
        xs.push(x);
        ys.push(y);
    }

    let (xmin, xmax) = xs.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(mn, mx), &v| (mn.min(v), mx.max(v)));
    let (ymin, ymax) = ys.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(mn, mx), &v| (mn.min(v), mx.max(v)));

    let mut chart = ChartBuilder::on(&root)
        .caption("CSV scatter: sepal_length vs petal_length (Iris)", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(70)
        .build_cartesian_2d((xmin - 0.2)..(xmax + 0.2), (ymin - 0.2)..(ymax + 0.2))?;

    draw_mesh_f64(&mut chart, "sepal_length", "petal_length")?;

    let mut rng = rand::rng();
    chart.draw_series(xs.iter().zip(ys.iter()).map(|(&x, &y)| {
        // Slight random styling variation so the plot has more depth
        let radius = 3 + (rng.random_range(0..=2) as i32);
        Circle::new((x, y), radius, BLUE.mix(0.55).filled())
    }))?;

    root.present()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    ensure_output_dir()?;

    example_1()?;
    example_2()?;
    example_3()?;
    example_4()?;
    example_5()?;
    example_6()?;
    example_7_csv_plot()?;

    Ok(())
}
