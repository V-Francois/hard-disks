use crate::geometry;
use crate::state;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn write_coords_to_file(state: &state::State, filepath: &Path) {
    let mut file = File::create(&filepath).unwrap();
    writeln!(
        file,
        "{} {} {} {}",
        state.disks.len(),
        state.disks[0].radius,
        state.sim_box.lx,
        state.sim_box.ly
    )
    .unwrap();
    for disk in state.disks.iter() {
        writeln!(
            file,
            "{} {}",
            geometry::put_in_box_x(disk.position.x, &state.sim_box),
            geometry::put_in_box_y(disk.position.y, &state.sim_box),
        )
        .unwrap();
    }
}
