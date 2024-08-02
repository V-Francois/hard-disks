use std::env;
use std::path;

pub mod config;
pub mod disks;
pub mod geometry;
pub mod io;
pub mod sample;
pub mod state;

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

    //let mut state = state::State::hexagonal_packing(10, 10, 0.8);
    //
    //let filepath = path::Path::new("initial.txt");
    //io::write_coords_to_file(&state, filepath);
    //
    //let nb_steps = 1e7 as u32;
    //let acceptance_rate = sample::sample_npt(&mut state, 10.0, nb_steps);
    ////let acceptance_rate = sample::sample_nvt(&mut state, nb_steps);
    //println!("Acceptance rate: {}", acceptance_rate);
    //
    //let filepath = path::Path::new("final.txt");
    //io::write_coords_to_file(&state, filepath);
}
