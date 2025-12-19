use plotters::coord::Shift;
use plotters::coord::types::RangedCoordf64;
use plotters::prelude::*;
use rand::prelude::*;
use rand_distr::{Distribution, Normal};
use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use std::thread;
use std::time::Duration;

const FIG_300DPI: (u32, u32) = (2400, 1600);       // ~8x5.33 inches @ 300 DPI
const GRID_2X3_300DPI: (u32, u32) = (3600, 2400);  // larger for 2x3 comparisons

fn ensure_output_dir() -> Result<(), Box<dyn Error>> {
    fs::create_dir_all("output")?;
    Ok(())
}

fn with_png_root(path: &str, size: (u32, u32)) -> Result<DrawingArea<BitMapBackend<'_>, Shift>, Box<dyn Error>> {
    let root = BitMapBackend::new(path, size).into_drawing_area();
    root.fill(&WHITE)?;
    Ok(root)
}

fn randn(n: usize, mu: f64, sigma: f64) -> Vec<f64> {
    let mut rng = rand::rng();
    let dist = Normal::new(mu, sigma).unwrap();
    (0..n).map(|_| dist.sample(&mut rng)).collect()
}

fn mean(x: &[f64]) -> f64 {
    x.iter().sum::<f64>() / (x.len().max(1) as f64)
}

fn std_dev(x: &[f64]) -> f64 {
    if x.len() < 2 {
        return 0.0;
    }
    let m = mean(x);
    let var = x.iter().map(|v| (v - m) * (v - m)).sum::<f64>() / ((x.len() - 1) as f64);
    var.sqrt()
}

fn quantile(sorted: &[f64], q: f64) -> f64 {
    if sorted.is_empty() {
        return f64::NAN;
    }
    let n = sorted.len() as f64;
    let pos = (n - 1.0) * q;
    let lo = pos.floor() as usize;
    let hi = pos.ceil() as usize;
    if lo == hi {
        sorted[lo]
    } else {
        let w = pos - (lo as f64);
        sorted[lo] * (1.0 - w) + sorted[hi] * w
    }
}

fn iqr(x: &[f64]) -> f64 {
    let mut v = x.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    quantile(&v, 0.75) - quantile(&v, 0.25)
}

fn data_min_max(x: &[f64]) -> (f64, f64) {
    let mut mn = f64::INFINITY;
    let mut mx = f64::NEG_INFINITY;
    for &v in x {
        mn = mn.min(v);
        mx = mx.max(v);
    }
    if mn == mx {
        (mn - 1.0, mx + 1.0)
    } else {
        (mn, mx)
    }
}

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

fn edges_from_bins(minv: f64, maxv: f64, bins: usize) -> Vec<f64> {
    linspace(minv, maxv, bins + 1)
}

fn counts_from_edges(data: &[f64], edges: &[f64]) -> Vec<usize> {
    let nb = edges.len().saturating_sub(1);
    let mut counts = vec![0usize; nb];

    for &v in data {
        for i in 0..nb {
            let lo = edges[i];
            let hi = edges[i + 1];
            let in_bin = if i == nb - 1 {
                v >= lo && v <= hi
            } else {
                v >= lo && v < hi
            };
            if in_bin {
                counts[i] += 1;
                break;
            }
        }
    }
    counts
}

#[derive(Clone, Copy)]
enum Normalization {
    Count,
    CountDensity,
    Probability,
    Pdf,
}

fn normalized_heights(counts: &[usize], edges: &[f64], norm: Normalization) -> Vec<f64> {
    let n = counts.iter().sum::<usize>().max(1) as f64;
    let mut heights = Vec::with_capacity(counts.len());

    for i in 0..counts.len() {
        let c = counts[i] as f64;
        let w = (edges[i + 1] - edges[i]).abs().max(1e-12);
        let h = match norm {
            Normalization::Count => c,
            Normalization::CountDensity => c / w,
            Normalization::Probability => c / n,
            Normalization::Pdf => c / (n * w),
        };
        heights.push(h);
    }
    heights
}

fn draw_histogram(
    root: &DrawingArea<BitMapBackend<'_>, Shift>,
    title: &str,
    x_label: &str,
    y_label: &str,
    edges: &[f64],
    heights: &[f64],
    style: ShapeStyle,
) -> Result<(), Box<dyn Error>> {
    let (xmin, xmax) = (*edges.first().unwrap(), *edges.last().unwrap());
    let ymax = heights
        .iter()
        .cloned()
        .fold(0.0f64, |a, b| a.max(b))
        .max(1e-12);

    let mut chart = ChartBuilder::on(root)
        .caption(title, ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(80)
        .build_cartesian_2d(xmin..xmax, 0.0f64..(ymax * 1.1))?;

    chart
        .configure_mesh()
        .x_desc(x_label)
        .y_desc(y_label)
        .label_style(("sans-serif", 24))
        .axis_desc_style(("sans-serif", 26))
        .draw()?;

    for i in 0..heights.len() {
        let x0 = edges[i];
        let x1 = edges[i + 1];
        let h = heights[i];
        chart.draw_series(std::iter::once(Rectangle::new(
            [(x0, 0.0), (x1, h)],
            style.clone().filled(),
        )))?;
    }

    Ok(())
}

fn bins_sturges(n: usize) -> usize {
    let n = n.max(1) as f64;
    (n.log2() + 1.0).ceil().max(1.0) as usize
}

fn bins_sqrt(n: usize) -> usize {
    ((n.max(1) as f64).sqrt().ceil().max(1.0)) as usize
}

fn bins_scott(data: &[f64]) -> usize {
    let n = data.len().max(2) as f64;
    let (mn, mx) = data_min_max(data);
    let sd = std_dev(data).max(1e-12);
    let bw = 3.5 * sd / n.powf(1.0 / 3.0);
    let k = ((mx - mn) / bw).ceil().max(1.0) as usize;
    k
}

fn bins_fd(data: &[f64]) -> usize {
    let n = data.len().max(2) as f64;
    let (mn, mx) = data_min_max(data);
    let i = iqr(data).max(1e-12);
    let bw = 2.0 * i / n.powf(1.0 / 3.0);
    let k = ((mx - mn) / bw).ceil().max(1.0) as usize;
    k
}

fn bins_auto(data: &[f64]) -> usize {
    let k1 = bins_sturges(data.len());
    let k2 = bins_fd(data);
    k1.max(k2).max(1)
}

// 1) Simple histogram of standard normal data with automatic binning
fn example_1() -> Result<(), Box<dyn Error>> {
    let x1 = randn(10_000, 0.0, 1.0);
    let (mn, mx) = data_min_max(&x1);

    let bins = bins_auto(&x1);
    println!("Histogram with {bins} bins");

    let edges = edges_from_bins(mn, mx, bins);
    let counts = counts_from_edges(&x1, &edges);
    let heights = normalized_heights(&counts, &edges, Normalization::Count);

    let root = with_png_root("output/histogram_1.png", FIG_300DPI)?;
    draw_histogram(
        &root,
        "Histogram of standard normal data",
        "Value",
        "Frequency",
        &edges,
        &heights,
        BLUE.mix(0.55).stroke_width(1),
    )?;
    root.present()?;
    Ok(())
}

// 2) Compare binning rules with 2x3 panels
fn example_2() -> Result<(), Box<dyn Error>> {
    let x2 = randn(10_000, 0.0, 1.0);
    let (mn, mx) = data_min_max(&x2);

    let root = with_png_root("output/histogram_2.png", GRID_2X3_300DPI)?;
    let areas = root.split_evenly((2, 3));

    let rules: Vec<(&str, usize)> = vec![
        ("Automatic binning", bins_auto(&x2)),
        ("Scott's rule", bins_scott(&x2)),
        ("Freedman-Diaconis rule", bins_fd(&x2)),
        ("Integers rule", ((mx.ceil() - mn.floor()).max(1.0) as usize)),
        ("Sturges' rule", bins_sturges(x2.len())),
        ("Square root rule", bins_sqrt(x2.len())),
    ];

    for (i, (title, bins)) in rules.iter().enumerate() {
        let edges = if *title == "Integers rule" {
            let lo = mn.floor();
            let hi = mx.ceil();
            let mut e = Vec::new();
            let mut v = lo;
            while v <= hi + 1e-9 {
                e.push(v);
                v += 1.0;
            }
            if e.len() < 2 {
                vec![lo, lo + 1.0]
            } else {
                e
            }
        } else {
            edges_from_bins(mn, mx, (*bins).max(1))
        };

        let counts = counts_from_edges(&x2, &edges);
        let heights = normalized_heights(&counts, &edges, Normalization::Count);

        draw_histogram(
            &areas[i],
            title,
            "Value",
            "Frequency",
            &edges,
            &heights,
            Palette99::pick(i).mix(0.55).stroke_width(1),
        )?;
    }

    root.present()?;
    Ok(())
}

// 3) Demonstrate changing the number of bins (writes multiple images)
fn example_3() -> Result<(), Box<dyn Error>> {
    let x3 = randn(1000, 0.0, 1.0);
    let (mn, mx) = data_min_max(&x3);

    // Initial (auto)
    let bins0 = bins_auto(&x3);
    {
        let edges = edges_from_bins(mn, mx, bins0);
        let counts = counts_from_edges(&x3, &edges);
        let heights = normalized_heights(&counts, &edges, Normalization::Count);

        let root = with_png_root("output/histogram_3_step_0.png", FIG_300DPI)?;
        draw_histogram(
            &root,
            &format!("{bins0} bins"),
            "Value",
            "Frequency",
            &edges,
            &heights,
            BLUE.mix(0.55).stroke_width(1),
        )?;
        root.present()?;
    }

    // Pause briefly (optional)
    thread::sleep(Duration::from_millis(800));

    // Updated to fixed bin count (50)
    let bins1 = 50usize;
    {
        let edges = edges_from_bins(mn, mx, bins1);
        let counts = counts_from_edges(&x3, &edges);
        let heights = normalized_heights(&counts, &edges, Normalization::Count);

        let root = with_png_root("output/histogram_3_step_1.png", FIG_300DPI)?;
        draw_histogram(
            &root,
            &format!("{bins1} bins"),
            "Value",
            "Frequency",
            &edges,
            &heights,
            RED.mix(0.55).stroke_width(1),
        )?;
        root.present()?;
    }

    Ok(())
}

// 4) Histogram with custom bin edges and count-density normalization
fn example_4() -> Result<(), Box<dyn Error>> {
    let x4 = randn(10_000, 0.0, 1.0);

    let edges: Vec<f64> = vec![
        -10.0, -2.0, -1.75, -1.5, -1.25, -1.0, -0.75, -0.5, -0.25, 0.0, 0.25, 0.5, 0.75, 1.0,
        1.25, 1.5, 1.75, 2.0, 10.0,
    ];

    let counts = counts_from_edges(&x4, &edges);
    let heights = normalized_heights(&counts, &edges, Normalization::CountDensity);

    let root = with_png_root("output/histogram_4.png", FIG_300DPI)?;
    draw_histogram(
        &root,
        "Histogram with custom bin edges",
        "Value",
        "Count density",
        &edges,
        &heights,
        GREEN.mix(0.55).stroke_width(1),
    )?;
    root.present()?;
    Ok(())
}

// 5) Categorical histogram (bar chart) for discrete string categories
fn example_5() -> Result<(), Box<dyn Error>> {
    let categories: Vec<&str> = vec![
        "no", "no", "yes", "yes", "yes", "no", "no", "no", "no", "undecided", "undecided", "yes",
        "no", "no", "no", "yes", "no", "yes", "no", "yes", "no", "no", "no", "yes", "yes", "yes",
        "yes",
    ];

    let mut unique = vec!["no".to_string(), "yes".to_string(), "undecided".to_string()];
    unique.sort();
    unique.dedup();

    let mut counts = vec![0usize; unique.len()];
    for c in categories {
        if let Some(i) = unique.iter().position(|u| u == c) {
            counts[i] += 1;
        }
    }

    let ymax = counts.iter().cloned().max().unwrap_or(1) as i32;

    let root = with_png_root("output/histogram_5.png", FIG_300DPI)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Histogram of categorical responses", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(80)
        .build_cartesian_2d(0i32..(unique.len() as i32), 0i32..(ymax + 2))?;

    chart
        .configure_mesh()
        .x_desc("Category")
        .y_desc("Count")
        .x_labels(unique.len())
        .x_label_formatter(&|v| {
            let i = (*v as usize).min(unique.len().saturating_sub(1));
            unique[i].clone()
        })
        .label_style(("sans-serif", 24))
        .axis_desc_style(("sans-serif", 26))
        .draw()?;

    for (i, &c) in counts.iter().enumerate() {
        let x0 = i as i32;
        let x1 = x0 + 1;
        chart.draw_series(std::iter::once(Rectangle::new(
            [(x0, 0), (x1, c as i32)],
            BLUE.mix(0.55).filled(),
        )))?;
    }

    root.present()?;
    Ok(())
}

// 6) Overlay normalized histograms for two different normal distributions
fn example_6() -> Result<(), Box<dyn Error>> {
    let x = randn(2000, 0.0, 1.0);
    let y = randn(5000, 1.0, 1.0);

    let (mn1, mx1) = data_min_max(&x);
    let (mn2, mx2) = data_min_max(&y);
    let mn = mn1.min(mn2);
    let mx = mx1.max(mx2);

    let bin_width = 0.25;
    let lo = (mn / bin_width).floor() * bin_width;
    let hi = (mx / bin_width).ceil() * bin_width;

    let mut edges = Vec::new();
    let mut v = lo;
    while v <= hi + 1e-9 {
        edges.push(v);
        v += bin_width;
    }
    if edges.len() < 2 {
        edges = vec![lo, lo + bin_width];
    }

    let cx = counts_from_edges(&x, &edges);
    let cy = counts_from_edges(&y, &edges);

    let hx = normalized_heights(&cx, &edges, Normalization::Probability);
    let hy = normalized_heights(&cy, &edges, Normalization::Probability);

    let root = with_png_root("output/histogram_6.png", FIG_300DPI)?;

    // Draw first
    draw_histogram(
        &root,
        "Overlaid normalized histograms",
        "Value",
        "Probability",
        &edges,
        &hx,
        BLUE.mix(0.40).stroke_width(1),
    )?;

    // Overlay second by drawing rectangles again
    {
        let (xmin, xmax) = (*edges.first().unwrap(), *edges.last().unwrap());
        let ymax = hx
            .iter()
            .chain(hy.iter())
            .cloned()
            .fold(0.0f64, |a, b| a.max(b))
            .max(1e-12);

        let mut chart = ChartBuilder::on(&root)
            .caption("Overlaid normalized histograms", ("sans-serif", 40))
            .margin(20)
            .x_label_area_size(60)
            .y_label_area_size(80)
            .build_cartesian_2d(xmin..xmax, 0.0f64..(ymax * 1.1))?;

        chart
            .configure_mesh()
            .x_desc("Value")
            .y_desc("Probability")
            .label_style(("sans-serif", 24))
            .axis_desc_style(("sans-serif", 26))
            .draw()?;

        // Blue layer
        for i in 0..hx.len() {
            chart.draw_series(std::iter::once(Rectangle::new(
                [(edges[i], 0.0), (edges[i + 1], hx[i])],
                BLUE.mix(0.40).filled(),
            )))?;
        }
        // Red layer
        for i in 0..hy.len() {
            chart.draw_series(std::iter::once(Rectangle::new(
                [(edges[i], 0.0), (edges[i + 1], hy[i])],
                RED.mix(0.40).filled(),
            )))?;
        }
    }

    root.present()?;
    Ok(())
}

// 7) Histogram normalized to PDF overlaid with theoretical normal distribution
fn example_7() -> Result<(), Box<dyn Error>> {
    let x = randn(5000, 5.0, 2.0);
    let (mn, mx) = data_min_max(&x);

    let bins = bins_auto(&x);
    let edges = edges_from_bins(mn, mx, bins);
    let counts = counts_from_edges(&x, &edges);
    let heights = normalized_heights(&counts, &edges, Normalization::Pdf);

    let mu = 5.0;
    let sigma = 2.0;
    let inv = 1.0 / (sigma * (2.0 * std::f64::consts::PI).sqrt());

    let root = with_png_root("output/histogram_7.png", FIG_300DPI)?;
    let (xmin, xmax) = (*edges.first().unwrap(), *edges.last().unwrap());
    let ymax_hist = heights.iter().cloned().fold(0.0f64, |a, b| a.max(b));
    let ymax_pdf = inv;
    let ymax = ymax_hist.max(ymax_pdf).max(1e-12);

    let mut chart = ChartBuilder::on(&root)
        .caption("Histogram with theoretical normal PDF", ("sans-serif", 40))
        .margin(20)
        .x_label_area_size(60)
        .y_label_area_size(80)
        .build_cartesian_2d(xmin..xmax, 0.0f64..(ymax * 1.2))?;

    chart
        .configure_mesh()
        .x_desc("Value")
        .y_desc("Probability density")
        .label_style(("sans-serif", 24))
        .axis_desc_style(("sans-serif", 26))
        .draw()?;

    // PDF-normalized histogram
    for i in 0..heights.len() {
        chart.draw_series(std::iter::once(Rectangle::new(
            [(edges[i], 0.0), (edges[i + 1], heights[i])],
            BLUE.mix(0.45).filled(),
        )))?;
    }

    // Theoretical PDF curve
    let npts = 600usize;
    let series = (0..=npts).map(|i| {
        let t = i as f64 / (npts as f64);
        let xx = xmin + t * (xmax - xmin);
        let z = (xx - mu) / sigma;
        let yy = (-0.5 * z * z).exp() * inv;
        (xx, yy)
    });
    chart.draw_series(LineSeries::new(series, RED.stroke_width(3)))?;

    root.present()?;
    Ok(())
}

// --- CSV helper ---
fn download_csv_text(url: &str) -> Result<String, Box<dyn Error>> {
    let resp = reqwest::blocking::get(url)?;
    let text = resp.text()?;

    // Normalize whitespace-separated records into newline-separated CSV rows if needed.
    let normalized = text.split_whitespace().collect::<Vec<_>>().join("\n");
    Ok(normalized)
}

// 8) CSV-driven histogram
fn example_8_csv_hist() -> Result<(), Box<dyn Error>> {
    let url = "https://raw.githubusercontent.com/mohammadijoo/Datasets/refs/heads/main/iris.csv";

    // If the CSV is on your local drive:
    // let mut rdr = csv::Reader::from_path("path/to/iris.csv")?;

    let csv_text = download_csv_text(url)?;
    let mut rdr = csv::Reader::from_reader(std::io::Cursor::new(csv_text));

    let headers = rdr.headers()?.clone();
    let idx = headers
        .iter()
        .position(|h| h == "sepal_length")
        .ok_or("Missing column: sepal_length")?;

    let mut values = Vec::<f64>::new();
    for rec in rdr.records() {
        let rec = rec?;
        let v: f64 = rec.get(idx).ok_or("Bad record")?.trim().parse()?;
        values.push(v);
    }

    let (mn, mx) = data_min_max(&values);
    let bins = bins_auto(&values);
    let edges = edges_from_bins(mn, mx, bins);
    let counts = counts_from_edges(&values, &edges);
    let heights = normalized_heights(&counts, &edges, Normalization::Count);

    let root = with_png_root("output/histogram_8_csv.png", FIG_300DPI)?;
    draw_histogram(
        &root,
        "CSV histogram: sepal_length (Iris)",
        "sepal_length",
        "Frequency",
        &edges,
        &heights,
        GREEN.mix(0.55).stroke_width(1),
    )?;
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
    example_7()?;
    example_8_csv_hist()?;

    Ok(())
}
