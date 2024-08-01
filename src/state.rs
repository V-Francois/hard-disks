use crate::disks;
use crate::geometry;

pub struct State {
    pub disks: Vec<disks::Disk>,
    pub grid: geometry::Grid,
    pub sim_box: geometry::Box,
}

pub fn create_simple_state(n_disks: u32) -> State {
    let radius = 0.5;
    let mut disks: Vec<disks::Disk> = Vec::new();
    let n_per_side = (n_disks as f64).sqrt().ceil() as u32;
    let mut counter = 0;
    for i in 0..n_per_side {
        for j in 0..n_per_side {
            let pos = geometry::Position {
                x: (i * 2) as f64,
                y: (j * 2) as f64,
            };
            let new_disk = disks::Disk {
                position: pos,
                radius: radius,
                cell_id: 0,
            };
            disks.push(new_disk);
            counter += 1;
            if counter == n_disks {
                break;
            }
        }
    }

    let sim_box = geometry::Box { lx: 20.0, ly: 20.0 };

    // create grid list
    let grid = geometry::create_grid(&mut disks, &sim_box);

    return State {
        disks: disks,
        grid: grid,
        sim_box: sim_box,
    };
}
