pub trait Sample where Self: Sized + cpal::Sample {
    fn aggregate_sample(self) -> bool;
}

impl Sample for i16 {
    fn aggregate_sample(self) -> bool {
        (self as u16).aggregate_sample()
    }
}

impl Sample for u16 {
    fn aggregate_sample(self) -> bool {
        let mut val = self;
        let mut res = val;
        for _ in 1..16 {
            val <<= 1;
            res ^= val;
        }
        res & 1 != 0
    }
}

impl Sample for f32 {
    fn aggregate_sample(self) -> bool {
        let mut val = u32::from_le_bytes(self.to_ne_bytes());
        let mut res = val;
        for _ in 1..32 {
            val >>= 1;
            res ^= val;
        }
        res & 1 != 0
    }
}
