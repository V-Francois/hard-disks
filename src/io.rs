use crate::disks::Disk;
use crate::geometry;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write_coords_to_file(disks: &Vec<Disk>, sim_box: &geometry::Box, filepath: &Path) {
    let mut file = File::create(&filepath).unwrap();
    writeln!(
        file,
        "{} {} {} {}",
        disks.len(),
        disks[0].radius,
        sim_box.lx,
        sim_box.ly
    )
    .unwrap();
    for disk in disks.iter() {
        writeln!(file, "{} {}", disk.position.x, disk.position.y).unwrap();
    }
}
