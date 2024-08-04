use std::env;
use std::path;

pub mod config;
pub mod disks;
pub mod geometry;
pub mod sample;
pub mod state;
pub mod thermo;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Usage: {} config.yaml", args[0]);
    }

    let file_path = &args[1];
    if !path::Path::new(file_path).exists() {
        panic!("Path {} doesn’t exist", file_path);
    }

    let config = config::Config::from_yaml_file(file_path.as_str());

    let disk_each_direction = (config.n_disk as f64).sqrt().floor() as u32;
    if config.n_disk != disk_each_direction * disk_each_direction {
        panic!("Number of disk must be a perfect square");
    }
    let mut state = state::State::hexagonal_packing(
        disk_each_direction,
        disk_each_direction,
        config.packing_fraction,
    );

    let filepath = path::Path::new("initial.txt");
    state.write_coords_to_file(filepath);

    let thermo: thermo::Thermo;
    if let Some(pressure) = config.pressure {
        thermo = sample::sample_npt(&mut state, pressure, config.n_step);
        let filepath = path::Path::new("density.csv");
        thermo.density_to_csv(filepath);
    } else {
        thermo = sample::sample_nvt(&mut state, config.n_step);
    }

    let filepath = path::Path::new("final.txt");
    state.write_coords_to_file(filepath);
}
