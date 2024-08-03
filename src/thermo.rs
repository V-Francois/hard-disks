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
}
