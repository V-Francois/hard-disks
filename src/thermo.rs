use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct Thermo {
    pub step: Vec<u32>,
    pub density: Vec<f64>,
    pub nvt_acceptance_rate: f64,
    pub npt_acceptance_rate: f64,
}

impl Thermo {
    pub fn empty_thermo() -> Thermo {
        return Thermo {
            step: Vec::new(),
            density: Vec::new(),
            nvt_acceptance_rate: 0.0,
            npt_acceptance_rate: 0.0,
        };
    }

    pub fn to_csv(&self, filepath: &Path) {
        let mut file = File::create(&filepath).unwrap();
        writeln!(file, "step,density,nvt_acceptance_rate,npt_acceptance_rate",).unwrap();
        for i in 0..self.step.len() {
            writeln!(
                file,
                "{},{},{},{}",
                self.step[i], self.density[i], self.nvt_acceptance_rate, self.npt_acceptance_rate,
            )
            .unwrap();
        }
    }
}
