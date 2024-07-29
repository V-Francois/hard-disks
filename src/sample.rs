use rand;
use rand::Rng;

use crate::disks;
use crate::geometry;

pub fn sample_nvt(disks: &mut Vec<disks::Disk>, sim_box: &geometry::Box, nb_steps: u32) -> f64 {
    let max_displacement = 0.5;
    let mut rng = rand::thread_rng();
    let mut nb_success = 0;
    let nb_disks = disks.len() as u32;
    for _ in 0..nb_steps {
        let theta: f64 = rng.gen();
        let r: f64 = rng.gen();
        let dx = r * theta.cos() * max_displacement;
        let dy = r * theta.sin() * max_displacement;

        let disk_index: usize = rng.gen_range(0..nb_disks).try_into().unwrap();

        disks[disk_index].position.x += dx;
        disks[disk_index].position.y += dy;

        let mut overlaps = false;
        for j in 0..disks.len() {
            if j == disk_index {
                continue;
            }
            if disks::are_disks_overlapping(&disks[disk_index], &disks[j], &sim_box) {
                overlaps = true;
                break;
            }
        }

        if overlaps {
            disks[disk_index].position.x -= dx;
            disks[disk_index].position.y -= dy;
        } else {
            nb_success += 1;
        }
    }
    return (nb_success as f64) / (nb_steps as f64);
}
