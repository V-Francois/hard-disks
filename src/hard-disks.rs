use std::path;

pub mod disks;
pub mod geometry;
pub mod io;
pub mod sample;
pub mod state;

fn main() {
    let mut state = state::create_simple_state(100);

    let filepath = path::Path::new("initial.txt");
    io::write_coords_to_file(&state, filepath);

    let nb_steps = 1e7 as u32;
    let acceptance_rate = sample::sample_npt(&mut state.disks, &mut state.sim_box, 5.0, nb_steps);
    println!("Acceptance rate: {}", acceptance_rate);

    let filepath = path::Path::new("final.txt");
    io::write_coords_to_file(&state, filepath);
}
