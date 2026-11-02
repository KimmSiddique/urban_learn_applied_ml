pub(crate) mod config;
pub(crate) mod house_properties;
pub(crate) mod simulation;
pub(crate) mod statistics;

use config::Config;
use simulation::{experiment, simulation_manager::SimulationManager};
use std::{error::Error, io};

fn main() {
    if let Err(error) = run() {
        eprintln!("Experiment failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|a| a == "--help") {
        println!(
            "cargo run: interactive setup\ncargo run -- [--maps N] [--size N | --sizes N,N,...] [--steps N] [--limit N] [--output DIR] [--no-maps] [--interactive]"
        );
        println!(
            "Square sizes: 1-{}. --sizes supplies one size per map. Equal --width/--height values are also accepted.",
            config::MAX_MAP_SIZE
        );
        return Ok(());
    }
    let mut config = Config::parse(&args)?;
    if config.interactive {
        config.prompt(&mut io::stdin().lock(), &mut io::stdout().lock())?;
    }
    std::fs::create_dir_all(&config.output)?;
    println!(
        "Urban Learn | {} map(s), {} energy/agent, {} samples maximum",
        config.sizes.len(),
        config.steps,
        config.limit
    );
    println!(
        "One cardinal move costs one energy. Collection and training cost no movement energy."
    );
    println!(
        "MAE/RMSE: lower is better; R2: higher is better. Each map uses the same unseen test houses for all models."
    );
    let mut reports = Vec::new();
    for (index, &size) in config.sizes.iter().enumerate() {
        let map_id = index + 1;
        println!(
            "\nGenerating map {map_id}: {size} x {size} ({} tiles)",
            size * size
        );
        if config.print_maps && size > 50 {
            println!("Large map: full grids saved to TXT/PNG; console grids hidden above 50 x 50.");
        }
        let simulation = SimulationManager::new(size, size);
        let directory = config.output.join(format!("map_{map_id:03}"));
        reports.push(experiment::run(
            &simulation,
            map_id,
            config.steps,
            config.limit,
            &directory,
            config.print_maps && size <= 50,
        )?);
    }
    experiment::summarize(&reports, &config.output)?;
    println!(
        "\nReports, samples, predictions, maps and charts: {}",
        config.output.display()
    );
    Ok(())
}
