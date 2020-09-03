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
        self.count_ones() & 1 != 0
    }
}

impl Sample for f32 {
    fn aggregate_sample(self) -> bool {
        u32::from_le_bytes(self.to_ne_bytes()).count_ones() & 1 != 0
    }
}
