use rand;
use rand::Rng;

use crate::state;
use crate::thermo;

pub fn sample_nvt(state: &mut state::State, nb_steps: u32) -> f64 {
    let max_displacement = 0.5;
    let mut rng = rand::thread_rng();
    let mut nb_success = 0;
    let nb_disks = state.disks.len() as u32;
    for _ in 0..nb_steps {
        let theta: f64 = rng.gen();
        let r: f64 = rng.gen();
        let dx = r * theta.cos() * max_displacement;
        let dy = r * theta.sin() * max_displacement;

        let disk_index: usize = rng.gen_range(0..nb_disks).try_into().unwrap();

        let old_x = state.disks[disk_index].position.x;
        let old_y = state.disks[disk_index].position.y;
        let new_x = old_x + dx;
        let new_y = old_y + dy;

        state.update_disk_coordinates(disk_index, new_x, new_y);

        let overlaps = state.is_disk_overlapping(disk_index);

        if overlaps {
            state.update_disk_coordinates(disk_index, old_x, old_y);
        } else {
            nb_success += 1;
        }
    }
    return (nb_success as f64) / (nb_steps as f64);
}

pub fn sample_npt(
    state: &mut state::State,
    pressure_over_kt: f64,
    nb_steps: u32,
) -> thermo::Thermo {
    let mut thermo = thermo::Thermo::empty_thermo();

    let number_of_sweeps_between_thermo_update = 100;

    // We’ll do N_disks NVT steps between attempts to change the volume
    let number_of_sweeps = (nb_steps as f32 / state.disks.len() as f32).ceil() as u32;

    let mut rng = rand::thread_rng();
    let mut nb_accepted = 0;
    let max_volume_ratio_percent = 0.5; // Allow +-1% changes
    let mut acceptance_nvt_sum = 0.0;
    for sweep_id in 0..number_of_sweeps {
        // Do a number of NVT step equal to the number of disks
        acceptance_nvt_sum += sample_nvt(state, state.disks.len() as u32);

        // Try to change the volume
        let ratio: f64 = 1.0 + (rng.gen::<f64>() - 0.5) * 2.0 * max_volume_ratio_percent / 100.0;

        // Select either x or y direction
        let change_along_x: bool = rng.gen_bool(0.5);

        // Compute proba now, before checking for overlap. Because if we say no, no need to check for overlaps
        let volume_before = state.sim_box.lx * state.sim_box.ly;
        let volume_after = state.sim_box.lx * ratio * state.sim_box.ly;

        let probability = (-(state.disks.len() as f64 * (volume_after.ln() - volume_before.ln())
            + pressure_over_kt * (volume_after - volume_before)))
            .exp();
        let mut accept_volume_change = false;
        // Probability is good, we’ll check for overlap
        if probability > rng.gen::<f64>() {
            if change_along_x {
                state.sim_box.lx *= ratio;
                for disk_id in 0..state.disks.len() {
                    state.update_disk_coordinates(
                        disk_id,
                        state.disks[disk_id].position.x * ratio,
                        state.disks[disk_id].position.y,
                    );
                }
            } else {
                state.sim_box.ly *= ratio;
                for disk_id in 0..state.disks.len() {
                    state.update_disk_coordinates(
                        disk_id,
                        state.disks[disk_id].position.x,
                        state.disks[disk_id].position.y * ratio,
                    );
                }
            }

            if state.are_any_disks_overlapping() {
                // Revert all the changes
                if change_along_x {
                    state.sim_box.lx /= ratio;
                    for disk_id in 0..state.disks.len() {
                        state.update_disk_coordinates(
                            disk_id,
                            state.disks[disk_id].position.x / ratio,
                            state.disks[disk_id].position.y,
                        );
                    }
                } else {
                    state.sim_box.ly /= ratio;
                    for disk_id in 0..state.disks.len() {
                        state.update_disk_coordinates(
                            disk_id,
                            state.disks[disk_id].position.x,
                            state.disks[disk_id].position.y / ratio,
                        );
                    }
                }
            } else {
                accept_volume_change = true;
            }
        }
        if accept_volume_change {
            // TODO: do a smarter thing
            state.update_grid();
            nb_accepted += 1;
        }

        // Record time series of thermo quantities
        if sweep_id % number_of_sweeps_between_thermo_update == 0 {
            thermo.step.push(sweep_id * state.disks.len() as u32);
            thermo.density.push(state.get_density());
        }
    }
    thermo.npt_acceptance_rate = nb_accepted as f64 / number_of_sweeps as f64;
    thermo.nvt_acceptance_rate = acceptance_nvt_sum as f64 / number_of_sweeps as f64;
    return thermo;
}
