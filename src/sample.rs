pub trait Sample where Self: Sized + cpal::Sample {
    fn aggregate_sample(self) -> bool;
    fn is_silent(self) -> bool;
}

impl Sample for i16 {

    fn aggregate_sample(self) -> bool {
        (self as u16).aggregate_sample()
    }

    fn is_silent(self) -> bool {
        self == 0
    }

}

impl Sample for u16 {

    fn aggregate_sample(self) -> bool {
        self.count_ones() & 1 != 0
    }

    fn is_silent(self) -> bool {
        self == 0
    }

}

impl Sample for f32 {

    fn aggregate_sample(self) -> bool {
        u32::from_le_bytes(self.to_ne_bytes()).count_ones() & 1 != 0
    }

    fn is_silent(self) -> bool {
        for &byte in self.to_ne_bytes().iter() {
            if byte == 1 {
                return false;
            }
        }
        return true;
    }

}
