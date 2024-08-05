use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct Thermo {
    pub step: Vec<u32>,
    pub density: Vec<f64>,
    pub nvt_acceptance_rate: f64,
    pub npt_acceptance_rate: f64,
    pub g_of_r: GofRlowR,
}

#[derive(Debug)]
pub struct GofRlowR {
    pub r: Vec<f64>,
    pub g: Vec<f64>,
    pub counters: Vec<u32>,
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

    pub fn density_to_csv(&self, filepath: &Path) {
        let mut file = File::create(&filepath).unwrap();
        writeln!(file, "step,density",).unwrap();
        for i in 0..self.step.len() {
            writeln!(file, "{},{}", self.step[i], self.density[i],).unwrap();
        }
    }
}

impl GofRlowR {
    pub fn empty_g_of_r() -> GofRlowR {
        return GofRlowR {
            r: Vec::new(),
            g: Vec::new(),
            counters: Vec::new(),
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
            self.g.push(0.0);
            self.counters.push(0);
        }
    }
}
