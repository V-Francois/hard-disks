use rand;
use rand::Rng;
use std::path;

pub mod disks;
pub mod geometry;
pub mod io;

fn main() {
    let radius = 0.5;
    let mut disks: Vec<disks::Disk> = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            let pos = geometry::Position {
                x: (i * 2) as f64,
                y: (j * 2) as f64,
            };
            let new_disk = disks::Disk {
                position: pos,
                radius: radius,
            };
            disks.push(new_disk);
        }
    }

    let sim_box = geometry::Box { lx: 20.0, ly: 20.0 };
    // check that the starting conf has no overlaps
    assert!(!disks::are_any_disks_overlapping(&disks, &sim_box));

    let filepath = path::Path::new("initial.txt");
    io::write_coords_to_file(&disks, &sim_box, filepath);

    let nb_steps = 10000;
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
    println!("Success rate: {}", (nb_success as f32) / (nb_steps as f32));

    let filepath = path::Path::new("final.txt");
    io::write_coords_to_file(&disks, &sim_box, filepath);
}
