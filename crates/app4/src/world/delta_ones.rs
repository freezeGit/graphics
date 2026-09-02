use crate::world::Rule;
use crate::world::emerge::BitArray;
use crate::world::step_bits;
use rand::Rng;
use statrs::statistics::Statistics;

const SIM_BITS: usize = 6000;
const DELTAS_VALS: usize = 250;

#[derive(Debug)]
pub struct Deltas {
    deltas: Vec<DeltaOnes>,
}

impl Deltas {
    pub fn new(rule: Rule, sample: u32, rng: &mut impl Rng) -> Self {
        //let mut deltas = Vec::<DeltaOnes>::with_capacity(251);
        let mut deltas = Vec::<DeltaOnes>::with_capacity(DELTAS_VALS + 1);
        for i in (0..=SIM_BITS).step_by(24) {
            deltas.push(DeltaOnes::new(rule, i, sample, rng));
        }

        Self { deltas }
    }

    pub fn len(&self) -> usize {
        self.deltas.len()
    }

    pub fn get_deltas(&self, i: usize) -> &DeltaOnes {
        assert!(i < self.deltas.len());
        &self.deltas[i]
    }
} // end impl Deltas

//----------------------------------------

#[derive(Debug, Copy, Clone)]
pub struct DeltaOnes {
    mean: f64,
    sem: f64,
}

impl DeltaOnes {
    fn new(rule: Rule, ones: usize, sample: u32, rng: &mut impl Rng) -> Self {
        assert!(ones <= SIM_BITS);
        assert!(sample > 0);

        let mut samples: Vec<f64> = Vec::with_capacity(sample as usize);
        for _ in 0..sample {
            let mut bits: BitArray = BitArray::new_with_random_ones(SIM_BITS, ones, rng);
            step_bits(&mut bits, rule, rng);
            let new_ones = bits.ones_count();
            let delta = new_ones as i32 - ones as i32;
            samples.push(delta as f64);
        }

        let data = samples.as_slice();

        let mean = data.mean();

        // let std_dev = data.std_dev();
        // let n = data.len() as f64;
        // let sem = std_dev / n.sqrt();
        let sem = 0.0;

        Self { mean, sem }
    }

    pub fn mean(&self) -> f64 {
        self.mean
    }

    pub fn sem(&self) -> f64 {
        self.sem
    }

    pub fn delta_stats_str(&self) -> String {
        format!(
            "mean: {:.2}, sem: {:.2},",
            self.mean,
            self.sem, //self.max_delta, self.min_delta
        )
    }
} // end impl DeltaOnes
