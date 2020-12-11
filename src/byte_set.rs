use IterState::*;

pub struct ByteSet {
    set: [u64; 4]
}

impl ByteSet {

    pub fn new() -> ByteSet {
        ByteSet { set: [0, 0, 0, 0] }
    }

    pub fn from_raw_parts(set: [u64; 4]) -> ByteSet {
        ByteSet { set }
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

    pub fn remove(&mut self, value: u8) {
        let (byte_idx, bit_idx) = ByteSet::locate(value);
        self.set[byte_idx as usize] &= !(1 << (bit_idx as u64));
    }

    pub fn difference(&mut self, other: &Self) {
        for i in 0..4 {
            self.set[i] &= !other.set[i];
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        for i in 0..4 {
            if self.set[i] & !other.set[i] != 0 {
                return false;
            }
        }
        return true;
    }

    pub fn is_empty(&self) -> bool {
        for i in 0..4 {
            if i != 0 {
                return false;
            }
        }
        return true;
    }

    pub fn iter(&self) -> Iter {
        return Iter {
            byte_set: &self.set,
            state: Start,
        };
    }

}

impl<'a> IntoIterator for &'a ByteSet {

    type Item = u8;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }

}

pub struct Iter<'a> {
    byte_set: &'a [u64; 4],
    state: IterState
}

enum IterState {
    Start,
    Next {
        val: u8,
        part: u64,
        byte_idx: u8,
        bit_idx: u8,
    },
    End,
}

impl<'a> Iterator for Iter<'a> {

    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let (mut val, mut part, mut byte_idx, mut bit_idx) = match &self.state {
            Start => (0, self.byte_set[0], 0, 0),
            &Next { val, part, byte_idx, bit_idx } => (val, part, byte_idx, bit_idx),
            End => {
                return None;
            },
        };
        loop {
            if (part & 1) == 0 {
                if bit_idx == 63 {
                    if byte_idx == 3 {
                        self.state = End;
                        return None;
                    }
                    bit_idx = 0;
                    byte_idx += 1;
                    part = self.byte_set[byte_idx as usize];
                }
                else {
                    bit_idx += 1;
                    part >>= 1;
                }
                val += 1;
                continue;
            }
            self.state =
                if bit_idx == 63 {
                    if byte_idx == 3 { End }
                    else {
                        Next {
                            val: val + 1,
                            part: self.byte_set[byte_idx as usize + 1],
                            byte_idx: byte_idx + 1,
                            bit_idx: 0,
                        }
                    }
                }
                else {
                    Next {
                        val: val + 1,
                        part: part >> 1,
                        byte_idx,
                        bit_idx: bit_idx + 1,
                    }
                };
            return Some(val);
        }
    }

}
