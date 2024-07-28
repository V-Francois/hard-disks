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
}
