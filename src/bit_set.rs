pub struct ByteSet {
    set: [u64; 4]
}

impl ByteSet {

    pub fn new() -> ByteSet {
        ByteSet { set: [0, 0, 0, 0] }
    }

    fn locate(value: u8) -> (u8, u8) {
        let byte_idx = value / 64;
        let bit_idx = value - byte_idx * 64;
        (byte_idx, bit_idx)
    }

    pub fn contains(&self, value: u8) -> bool {
        let (byte_idx, bit_idx) = ByteSet::locate(value);
        ((self.set[byte_idx as usize] >> (bit_idx as u64)) & 1) != 0
    }

    pub fn insert(&mut self, value: u8) {
        let (byte_idx, bit_idx) = ByteSet::locate(value);
        self.set[byte_idx as usize] |= 1 << (bit_idx as u64);
    }

}
