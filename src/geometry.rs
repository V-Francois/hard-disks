use crate::disks;

pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub struct Box {
    pub lx: f64,
    pub ly: f64,
}

pub struct Grid {
    pub nx: u32,
    pub ny: u32,
    pub cells: Vec<Cell>,
}

pub struct Cell {
    pub disk_ids: Vec<usize>,
    pub neighbor_ids: Vec<usize>,
}

pub fn distance_sq(pos_1: &Position, pos_2: &Position) -> f64 {
    let dx = pos_1.x - pos_2.x;
    let dy = pos_1.y - pos_2.y;
    return dx * dx + dy * dy;
}

pub fn distance_sq_periodic(pos_1: &Position, pos_2: &Position, sim_box: &Box) -> f64 {
    let mut dx = pos_1.x - pos_2.x;
    let mut dy = pos_1.y - pos_2.y;
    apply_boundary_conditions(&mut dx, &mut dy, sim_box);
    return dx * dx + dy * dy;
}

pub fn apply_boundary_conditions(dx: &mut f64, dy: &mut f64, sim_box: &Box) {
    if *dx > sim_box.lx / 2.0 {
        *dx -= sim_box.lx;
    } else if *dx < -sim_box.lx / 2.0 {
        *dx += sim_box.lx;
    }
    if *dy > sim_box.ly / 2.0 {
        *dy -= sim_box.ly;
    } else if *dy < -sim_box.ly / 2.0 {
        *dy += sim_box.ly;
    }
}

pub fn put_in_box_x(x: f64, sim_box: &Box) -> f64 {
    let mut new_x = x;
    if new_x > sim_box.lx {
        new_x -= sim_box.lx * (new_x / sim_box.lx).floor();
    } else if new_x < -0.0 {
        new_x += sim_box.lx * -(new_x / sim_box.lx).floor();
    }
    return new_x;
}

pub fn put_in_box_y(y: f64, sim_box: &Box) -> f64 {
    let mut new_y = y;
    if new_y > sim_box.ly {
        new_y -= sim_box.ly * (new_y / sim_box.ly).floor();
    } else if new_y < -0.0 {
        new_y += sim_box.ly * -(new_y / sim_box.ly).floor();
    }
    return new_y;
}

pub fn create_grid(disks: &Vec<disks::Disk>, sim_box: &Box) -> Grid {
    let dx_dy = disks[0].radius * 3.0;
    let nx: i32 = (sim_box.lx / dx_dy).floor() as i32;
    let ny: i32 = (sim_box.ly / dx_dy).floor() as i32;
    let number_of_cells = nx * ny;

    // Create empty array of cells
    let mut cells: Vec<Cell> = Vec::new();
    for _ in 0..number_of_cells {
        cells.push(Cell {
            disk_ids: Vec::new(),
            neighbor_ids: Vec::new(),
        });
    }

    let cell_id_from_idx_idy = |idx: i32, idy: i32| -> usize {
        let mut iidx = idx;
        let mut iidy = idy;
        if idx < 0 {
            iidx = nx - 1;
        } else if idx == nx {
            iidx = 0;
        }
        if idy < 0 {
            iidy = ny - 1;
        } else if idy == ny {
            iidy = 0;
        }
        return (iidx + nx * iidy) as usize;
    };
    // Fill the neighbors lists
    for idx in 0..nx {
        for idy in 0..ny {
            let id_here = cell_id_from_idx_idy(idx, idy);
            for offset_x in [-1, 0, 1] {
                for offset_y in [-1, 0, 1] {
                    if offset_x == 0 && offset_y == 0 {
                        continue;
                    }
                    cells[id_here as usize]
                        .neighbor_ids
                        .push(cell_id_from_idx_idy(idx + offset_x, idy + offset_y));
                }
            }
        }
    }

    // Now assign each disk to a cell
    let cell_id_from_xy = |x: f64, y: f64| -> usize {
        let ix = (x * (nx as f64) / sim_box.lx).floor() as i32;
        let iy = (y * (ny as f64) / sim_box.ly).floor() as i32;
        return (ix + nx * iy) as usize;
    };
    for i in 0..disks.len() {
        let cell_id = cell_id_from_xy(disks[i].position.x, disks[i].position.y);
        // TODO add to the disk itself
        cells[cell_id].disk_ids.push(cell_id);
    }

    return Grid {
        nx: nx as u32,
        ny: ny as u32,
        cells: cells,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        let pos_1 = Position { x: 0.0, y: 0.0 };
        let pos_2 = Position { x: 1.0, y: 2.0 };
        assert_eq!(distance_sq(&pos_1, &pos_2), 5.0);
    }

    #[test]
    fn test_distance_periodic() {
        let pos_1 = Position { x: 0.0, y: 0.0 };
        let pos_2 = Position { x: 0.0, y: 4.0 };
        let sim_box = Box { lx: 1.0, ly: 5.0 };
        assert_eq!(distance_sq_periodic(&pos_1, &pos_2, &sim_box), 1.0);
    }
}
