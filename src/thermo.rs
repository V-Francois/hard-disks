use crate::{geometry, state};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Thermo {
    pub step: Vec<u32>,
    pub density: Vec<f64>,
    pub nvt_acceptance_rate: f64,
    pub npt_acceptance_rate: f64,
    pub g_of_r: GofRlowR,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GofRlowR {
    pub r: Vec<f64>,
    pub g: Vec<u32>,
    counter: u32,
    r_max: f64,
    dr: f64,
    normalized_g: Vec<f64>,
}

impl Thermo {
    pub fn empty_thermo() -> Thermo {
        return Thermo {
            step: Vec::new(),
            density: Vec::new(),
            nvt_acceptance_rate: 0.0,
            npt_acceptance_rate: 0.0,
            g_of_r: GofRlowR::empty_g_of_r(),
        };
    }

    pub fn to_yaml(&self, filepath: &Path) {
        let file = File::create(&filepath).unwrap();
        //let f = std::fs::OpenOptions::new()
        //    .write(true)
        //    .create(true)
        //    .open(filepath)
        //    .expect("Couldn't open file");
        serde_yaml::to_writer(file, &self).unwrap();
    }
}

impl GofRlowR {
    pub fn empty_g_of_r() -> GofRlowR {
        return GofRlowR {
            r: Vec::new(),
            g: Vec::new(),
            counter: 0,
            r_max: 0.0,
            dr: 0.0,
            normalized_g: Vec::new(),
        };
    }

    pub fn initialize_vectors(&mut self, radius: f64) {
        // Go from (r - 2 sigma) / sigma = 0 to 0.1
        // (r_max - 2 sigma) = 0.1 sigma; r_max = 2.1 sigma
        // r_min = 2 sigma
        let n_points = 50;
        let dr = 0.1 * radius / n_points as f64;
        let mut r = 2.0 * radius + dr / 2.0;
        for _ in 0..n_points {
            self.r.push(r);
            r += dr;
            self.g.push(0);
            self.normalized_g.push(0.0);
        }
        self.counter = 0;
        self.r_max = r - dr / 2.0;
        self.dr = dr;
    }

    pub fn update(&mut self, state: &state::State) {
        let sigma = state.disks[0].radius;
        for disk_id_i in 0..state.disks.len() {
            for disk_id_j in state.get_neighbor_disks(disk_id_i) {
                let r_ij = geometry::distance_sq_periodic(
                    &state.disks[disk_id_i].position,
                    &state.disks[disk_id_j].position,
                    &state.sim_box,
                )
                .sqrt();
                if r_ij < self.r_max {
                    let g_id = ((r_ij - 2.0 * sigma) / self.dr).floor() as u32;
                    self.g[g_id as usize] += 1;
                }
            }
        }
        self.counter += 1;
    }

    pub fn renormalize(&mut self, state: &state::State) {
        let number_density = state.get_number_density();
        for i in 0..self.g.len() {
            let r_low = self.r[i] - self.dr / 2.0;
            let r_high = r_low + self.dr;
            // surface area at that distance
            let surface_area = PI * (r_high * r_high - r_low * r_low);
            let expected_value = surface_area * number_density * state.disks.len() as f64;

            self.normalized_g[i] = self.g[i] as f64 / expected_value / self.counter as f64;
        }
    }
}
