use rand;
use rand::Rng;

use crate::disks;
use crate::geometry;
use crate::state;

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

        state.disks[disk_index].position.x += dx;
        state.disks[disk_index].position.y += dy;

        let mut overlaps = false;
        for j in 0..state.disks.len() {
            if j == disk_index {
                continue;
            }
            if disks::are_disks_overlapping(
                &state.disks[disk_index],
                &state.disks[j],
                &state.sim_box,
            ) {
                overlaps = true;
                break;
            }
        }

        if overlaps {
            state.disks[disk_index].position.x -= dx;
            state.disks[disk_index].position.y -= dy;
        } else {
            nb_success += 1;
        }
    }
    return (nb_success as f64) / (nb_steps as f64);
}

pub fn sample_npt(state: &mut state::State, pressure_over_kt: f64, nb_steps: u32) -> f64 {
    // We’ll do N_disks NVT steps between attempts to change the volume
    let number_of_sweeps = (nb_steps as f32 / state.disks.len() as f32).ceil() as u32;

    let mut rng = rand::thread_rng();
    let mut nb_accepted = 0;
    let max_volume_ratio_percent = 0.5; // Allow +-1% changes
    for _ in 0..number_of_sweeps {
        // Do a number of NVT step equal to the number of disks
        _ = sample_nvt(state, state.disks.len() as u32);

        // Try to change the volume
        let ratio: f64 = 1.0 + (rng.gen::<f64>() - 0.5) * 2.0 * max_volume_ratio_percent / 100.0;

        // Select either x or y direction
        let change_along_x: bool = rng.gen_bool(0.5);

        // Compute proba now, before checking for overlap. Because if we say no, no need to check for overlaps
        let volume_before = state.sim_box.lx * state.sim_box.ly;
        let volume_after: f64;
        if change_along_x {
            volume_after = state.sim_box.lx * ratio * state.sim_box.ly;
        } else {
            volume_after = state.sim_box.lx * state.sim_box.ly * ratio;
        }
        let probability = (-(state.disks.len() as f64 * (volume_after.ln() - volume_before.ln())
            + pressure_over_kt * (volume_after - volume_before)))
            .exp();
        let mut accept_volume_change = false;
        // Probability is good, we’ll check for overlap
        if probability > rng.gen::<f64>() {
            if change_along_x {
                state.sim_box.lx *= ratio;
                for disk in state.disks.iter_mut() {
                    disk.position.x =
                        geometry::put_in_box_x(disk.position.x, &state.sim_box) * ratio;
                }
            } else {
                state.sim_box.ly *= ratio;
                for disk in state.disks.iter_mut() {
                    disk.position.y =
                        geometry::put_in_box_y(disk.position.y, &state.sim_box) * ratio;
                }
            }

            if disks::are_any_disks_overlapping(&state.disks, &state.sim_box) {
                // Revert all the changes
                if change_along_x {
                    state.sim_box.lx /= ratio;
                    for disk in state.disks.iter_mut() {
                        disk.position.x /= ratio;
                    }
                } else {
                    state.sim_box.ly /= ratio;
                    for disk in state.disks.iter_mut() {
                        disk.position.y /= ratio;
                    }
                }
            } else {
                accept_volume_change = true;
            }
        }
        if accept_volume_change {
            nb_accepted += 1;
        }
    }
    return (nb_accepted as f64) / (number_of_sweeps as f64);
}
