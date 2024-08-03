use std::f64::consts::PI;

use crate::disks;
use crate::geometry;

pub struct State {
    pub disks: Vec<disks::Disk>,
    pub sim_box: geometry::Box,
    grid: geometry::Grid,
}

impl State {
    pub fn get_neighbor_disks(&self, disk_id: usize) -> Vec<usize> {
        let mut neighbor_disks: Vec<usize> = Vec::new();
        let current_cell_id = self.disks[disk_id].cell_id;
        // All the disks in the same cell
        for id in self.grid.cells[current_cell_id].disk_ids.iter() {
            if *id != disk_id {
                neighbor_disks.push(*id);
            }
        }
        // The ones in the neighboring cells
        for cell_id in self.grid.cells[current_cell_id].neighbor_ids.iter() {
            for id in self.grid.cells[*cell_id].disk_ids.iter() {
                neighbor_disks.push(*id);
            }
        }
        return neighbor_disks;
    }

    pub fn is_disk_overlapping(&self, disk_id: usize) -> bool {
        for neighbor_id in self.get_neighbor_disks(disk_id) {
            if disks::are_disks_overlapping(
                &self.disks[disk_id],
                &self.disks[neighbor_id],
                &self.sim_box,
            ) {
                return true;
            }
        }
        return false;
    }

    pub fn are_any_disks_overlapping(&self) -> bool {
        for disk_id in 0..self.disks.len() {
            if self.is_disk_overlapping(disk_id) {
                return true;
            }
        }
        return false;
    }

    pub fn update_disk_coordinates(&mut self, disk_id: usize, new_x: f64, new_y: f64) {
        self.disks[disk_id].position.x = geometry::put_in_box_x(new_x, &self.sim_box);
        self.disks[disk_id].position.y = geometry::put_in_box_y(new_y, &self.sim_box);

        // Compute the cell id
        let ix = (self.disks[disk_id].position.x * (self.grid.nx as f64) / self.sim_box.lx).floor()
            as u32;
        let iy = (self.disks[disk_id].position.y * (self.grid.ny as f64) / self.sim_box.ly).floor()
            as u32;
        let new_cell_id = (ix + self.grid.nx * iy) as usize;
        if new_cell_id != self.disks[disk_id].cell_id {
            let old_cell_id = self.disks[disk_id].cell_id;
            // Remove the disk id from the old cell
            self.grid.cells[old_cell_id]
                .disk_ids
                .retain(|value| *value != disk_id);
            // Add it to the new one
            self.grid.cells[new_cell_id].disk_ids.push(disk_id);
            // And update on the disk
            self.disks[disk_id].cell_id = new_cell_id
        }
    }

    pub fn update_grid(&mut self) {
        self.grid = geometry::create_grid(&mut self.disks, &self.sim_box);
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

    pub fn hexagonal_packing(n_row: u32, n_column: u32, packing_fraction: f64) -> State {
        if packing_fraction < 0.0 || packing_fraction > 0.9 {
            panic!("Invalid packing fraction");
        }
        if n_row % 2 != 0 {
            panic!("Number of rows should be even");
        }
        if n_column % 2 != 0 {
            panic!("Number of columns should be even");
        }
        let number_of_disks = n_row * n_column;

        let radius = 0.5;
        let disk_volume = number_of_disks as f64 * radius * radius * PI;
        let box_volume = disk_volume / packing_fraction;

        let ly_over_lx = 3.0_f64.sqrt() / 2.0 * (n_row as f64) / (n_column as f64);
        // volume = lx * ly = ly/lx * lx² -> lx = sqrt(volume / ly_over_lx)
        let lx = (box_volume / ly_over_lx).sqrt();
        let ly = ly_over_lx * lx;

        let dx = lx / (n_column as f64);
        let dy = ly / (n_row as f64) * 2.0;

        let mut disks: Vec<disks::Disk> = Vec::new();
        let mut current_y = dy / 4.0;
        for row in 0..n_row {
            let mut current_x = dx / 4.0;
            if row % 2 == 1 {
                current_x += dx / 2.0;
            }
            for _ in 0..n_column {
                let pos = geometry::Position {
                    x: current_x,
                    y: current_y,
                };
                let new_disk = disks::Disk {
                    position: pos,
                    radius: radius,
                    cell_id: 0,
                };
                disks.push(new_disk);
                current_x += dx;
            }
            current_y += dy / 2.0;
        }
        let sim_box = geometry::Box { lx: lx, ly: ly };

        // create grid list
        let grid = geometry::create_grid(&mut disks, &sim_box);

        return State {
            disks: disks,
            grid: grid,
            sim_box: sim_box,
        };
    }

    pub fn get_density(&self) -> f64 {
        let radius = self.disks[0].radius;
        let disk_volume = self.disks.len() as f64 * radius * radius * PI;
        let box_volume = self.sim_box.lx * self.sim_box.ly;
        return disk_volume / box_volume;
    }
}
