use crate::world::Rule;
use crate::world::emerge::BitArray;
use crate::world::step_bits;
use rand::Rng;
use statrs::statistics::Statistics;

const SIM_BITS: usize = 6000;

#[derive(Debug)]
pub struct DeltaOnes {
    mean_delta: f64,
    max_delta: f64,
    min_delta: f64,
}

impl DeltaOnes {
    pub fn new(rule: Rule, ones: usize, sample: u32, rng: &mut impl Rng) -> Self {
        let mut samples: Vec<f64> = Vec::with_capacity(sample as usize);
        for _ in 0..sample {
            let mut bits: BitArray = BitArray::new_with_random_ones(SIM_BITS, ones, rng);
            step_bits(&mut bits, rule, rng);
            let new_ones = bits.ones_count();
            let delta = new_ones - ones ;
            samples.push(delta as f64);
        }

        let data = samples.as_slice();
        let mean_delta = data.mean();
        let max_delta = data.max();
        let min_delta = data.min();

        Self {
            mean_delta,
            max_delta,
            min_delta,
        }
    }

    pub fn mean_delta(&self) -> f64 {
        self.mean_delta
    }
    pub fn max_delta(&self) -> f64 {
        self.max_delta
    }
    pub fn min_delta(&self) -> f64 {
        self.min_delta
    }

    // TDJ: I don't like this. Do I want to return a tuple?
    pub fn delta_stats(&self) -> (f64, f64, f64) {
        (self.mean_delta, self.max_delta, self.min_delta)
    }

    pub fn delta_stats_str(&self) -> String {
        format!("mean: {:.2}, max: {:.2}, min: {:.2}", self.mean_delta, self.max_delta, self.min_delta)
    }
}


