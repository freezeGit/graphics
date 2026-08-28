use crate::world::Rule;
use crate::world::emerge::BitArray;
use crate::world::step_bits;
use rand::Rng;
use statrs::statistics::Statistics;

const SIM_BITS: usize = 6000;

#[derive(Debug)]
pub struct DeltaOnes {
    mean_delta: f64,
    sem_delta: f64,
    // max_delta: f64,
    // min_delta: f64,
}

impl DeltaOnes {
    pub fn new(rule: Rule, ones: usize, sample: u32, rng: &mut impl Rng) -> Self {
        assert!(ones <= SIM_BITS);
        assert!(sample > 1);

        let mut samples: Vec<f64> = Vec::with_capacity(sample as usize);
        for _ in 0..sample {
            let mut bits: BitArray = BitArray::new_with_random_ones(SIM_BITS, ones, rng);
            step_bits(&mut bits, rule, rng);
            let new_ones = bits.ones_count();
            //println!("new_ones: {}", new_ones);
            let delta = new_ones as i32 - ones as i32;
            //println!("delta: {}", delta);
            samples.push(delta as f64);
            //println!("samples: {:?}", samples);
        }

        //println!("samples: {:?}", samples);
        let data = samples.as_slice();

        let mean_delta = data.mean();

        let std_dev = data.std_dev();
        let n = data.len() as f64;
        let sem_delta = std_dev / n.sqrt();

        // let max_delta = data.max();
        // let min_delta = data.min();

        Self {
            mean_delta,
            sem_delta,
            // max_delta,
            // min_delta,
        }
    }

    pub fn mean_delta(&self) -> f64 {
        self.mean_delta
    }
    // pub fn max_delta(&self) -> f64 {
    //     self.max_delta
    // }
    // pub fn min_delta(&self) -> f64 {
    //     self.min_delta
    // }


    pub fn delta_stats_str(&self) -> String {
        format!(
            "mean: {:.2}, sem: {:.2},",
            self.mean_delta, self.sem_delta, //self.max_delta, self.min_delta
        )
    }

    // pub fn delta_stats_str(&self) -> String {
    //     format!(
    //         "mean: {:.2}, sem: {:.2}, max: {:.2}, min: {:.2}",
    //         self.mean_delta, self.sem_delta, self.max_delta, self.min_delta
    //     )
    // }
}
