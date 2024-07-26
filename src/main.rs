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
}
