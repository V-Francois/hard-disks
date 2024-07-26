pub mod disks;
pub mod geometry;

fn main() {
    let radius = 0.5;
    let mut disks: Vec<disks::Disk> = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            let pos = geometry::Position {
                x: i as f64,
                y: j as f64,
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
    for i in 0..disks.len() {
        for j in 0..disks.len() {
            if i == j {
                continue;
            }
            assert!(!disks::are_disks_overlapping(
                &disks[i], &disks[j], &sim_box
            ));
        }
    }
}
