pub struct CycleTime {
    /// Number of cycles passed
    num: u32,
    /// Frequency in HZ
    frequency: u32,
}

impl CycleTime {
    pub fn new(frequency: u32, cycles: u32) -> Self {
        CycleTime {
            num: cycles,
            frequency,
        }
    }

    pub fn micros(&self) -> u64 {
        let us = (1000000.0 * self.num as f64) / self.frequency as f64;
        us.round() as u64
    }

    // TODO: Add scaling function that can transform cycles to in this
    // frequency to cycles of a lower or higher one
    // pub fn scale(frequency: f64) -> usize {}
}

pub trait Timed {
    /// This function progresses the internal timings of a Timer
    fn catchup(&mut self, time: CycleTime);
}

#[cfg(test)]
mod tests {
    #[test]
    fn micros() {
        let ct = super::CycleTime {
            num: 3,
            frequency: 32768,
        };
        assert_eq!(ct.micros(), 92);
    }
}
