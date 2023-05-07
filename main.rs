use std::{
    env,
    error::Error,
    ffi::OsString,
    fs::File,
    process,
};

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    {
        let headers = rdr.headers()?;
        println!("{:?}", headers);
    }
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    let headers = rdr.headers()?;
    println!("{:?}", headers);
    Ok(())
}

use polars::prelude::*;
use polars::frame::DataFrame;
use std::path::Path;

fn read_data_frame_from_csv(
    csv_file_path: &Path,
) -> DataFrame {
    CsvReader::from_path(csv_file_path)
        .expect("Cannot open file.")
        .has_header(true)
        .finish()
        .unwrap()
}

let population_file_path: &Path = Path::new("earthdata/SYB65_1_202209_Population, Surface Area and Density.csv");
let population_df: DataFrame = read_data_frame_from_csv(population_file_path);


fn main() -> Result<()> {
    let df = DataFrame::read_csv("earthdata/SYB65_1_202209_Population, Surface Area and Density.csv")?;

    let df = df.drop("Series", "Footnotes", "Source");

    println!("{}", df);

    Ok(())
}

use rustlearn::cluster::KMeans;
use rustlearn::datasets::Dataset;

fn dropping() -> Result<()> {
    let df = df.drop("Series", "Footnotes", "Source");

    let data = Dataset::from(df);

    let n_clusters = 3;

    let mut kmeans = KMeans::new(n_clusters);

    kmeans.fit(&data).unwrap();

    let assignments = kmeans.predict(&data).unwrap();

    println!("{:?}", assignments);

    Ok(())
}

use network_analysis::{Network, DegreeCentrality, BetweennessCentrality, EigenvectorCentrality};

fn testing() -> Result<()> {
    let mut network = Network::new();
    for (src, dst) in df.iter_tuples::<(usize, usize)>() {
        network.add_edge(src, dst);
    }

    let degree_dist = network.degree_distribution();
    println!("Degree Distribution: {:?}", degree_dist);

    let degree_centrality = DegreeCentrality::new(&network);
    let node_degrees = degree_centrality.scores();
    println!("Degree Centrality: {:?}", node_degrees);

    let betweenness_centrality = BetweennessCentrality::new(&network);
    let node_betweenness = betweenness_centrality.scores();
    println!("Betweenness Centrality: {:?}", node_betweenness);

    let eigenvector_centrality = EigenvectorCentrality::new(&network);
    let node_eigenvector = eigenvector_centrality.scores();
    println!("Eigenvector Centrality: {:?}", node_eigenvector);

    Ok(())
}


use std::fs::File;
use std::error::Error;
use csv::ReaderBuilder;
use plotters::prelude::*;
use plotters::style::color::{Rgb};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path("SYB65_1_202209_Population, Surface Area and Density.csv")?;
    let mut data = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let country = record[0].to_string();
        let population = record[1].parse::<f64>().unwrap();
        data.push((country, population));
    }

    let colors = vec![Rgb(255, 255, 178), Rgb(254, 204, 92), Rgb(253, 141, 60), Rgb(240, 59, 32), Rgb(189, 0, 38)];
    let scale = LogScale::new(1.0, 1_000_000_000.0, colors);

    let root = BitMapBackend::new("population_map.png", (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .build_ranged(0f64..200f64, 0f64..120f64)?;

    for (country, population) in data {
        let color = scale.get_color(population);
        let polygon = get_country_polygon(&country)?;
        chart.draw_series(polygon.into_iter().map(|(x, y)| {
            Rectangle::new([(x, y), (x+1, y+1)], color.filled())
        }))?;
    }

    Ok(())
}

fn get_country_polygon(country: &str) -> Result<Vec<(f64, f64)>, Box<dyn Error>> {
    let polygons = include_str!("countries_polygons.txt");
    let polygon = polygons.lines()
        .find(|line| line.starts_with(country))
        .ok_or("Country not found")?;
    let points = polygon.split_at(polygon.find(':').unwrap()+1).1;
    let coords = points.split(',')
        .map(|s| s.trim().parse::<f64>().unwrap())
        .collect::<Vec<f64>>();
    let polygon = coords.chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<(f64, f64)>>();
    Ok(polygon)
}