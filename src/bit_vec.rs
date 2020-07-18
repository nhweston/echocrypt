use std::cell::RefCell;

/// Utility for efficiently building integers bit by bit.
pub struct BitVec {
    init: RefCell<Vec<u8>>,
    last: u8,
    crsr: u8,
}

impl BitVec {

    pub fn new(cap: usize) -> BitVec {
        BitVec {
            init: RefCell::new(Vec::with_capacity((cap / 8) + 1)),
            last: 0,
            crsr: 0
        }
    }

    pub fn push_zero(&mut self) {
        if self.crsr == 7 {
            self.init.borrow_mut().push(self.last);
            self.last = 0;
            self.crsr = 0;
        }
        else {
            self.crsr += 1;
        }
    }

    pub fn push_one(&mut self) {
        if self.crsr == 7 {
            self.last += 128;
            self.init.borrow_mut().push(self.last);
            self.last = 0;
            self.crsr = 0;
        }
        else {
            self.last += 1 << self.crsr;
            self.crsr += 1;
        }
    }

    pub fn len(&self) -> usize {
        self.init.borrow().len() * 8 + (self.crsr as usize)
    }

    pub fn reset(&mut self) -> Vec<u8> {
        let cap = self.init.borrow().capacity();
        let mut vec = self.init.replace(Vec::with_capacity(cap));
        vec.push(self.last);
        self.last = 0;
        self.crsr = 0;
        vec
    }

}

#[cfg(test)]
mod tests {

    use crate::bit_vec::BitVec;
    use num_bigint::BigUint;
    use num_traits::ToPrimitive;

    #[test]
    fn empty() {
        let mut bv = BitVec::new(0);
        let res = BigUint::from_bytes_le(bv.reset().as_ref());
        assert_eq!(res.to_u64().unwrap(), 0);
    }

    #[test]
    fn one() {
        let mut bv = BitVec::new(0);
        bv.push_one();
        let res = BigUint::from_bytes_le(bv.reset().as_ref());
        assert_eq!(res.to_u64().unwrap(), 1);
    }

    #[test]
    fn nine_digits() {
        let mut bv = BitVec::new(0);
        for _  in 0..9 { bv.push_one(); }
        let res = BigUint::from_bytes_le(bv.reset().as_ref());
        assert_eq!(res.to_u64().unwrap(), 0b111111111);
    }

    #[test]
    fn large() {
        let mut bv = BitVec::new(0);
        for _ in 0..7 { bv.push_zero(); }
        for _ in 0..6 { bv.push_one(); }
        for _ in 0..5 { bv.push_zero(); }
        for _ in 0..4 { bv.push_one(); }
        for _ in 0..3 { bv.push_zero(); }
        let res = BigUint::from_bytes_le(bv.reset().as_ref());
        assert_eq!(res.to_u64().unwrap(), 0b1111000001111110000000);
    }

    #[test]
    fn resets() {
        let mut bv = BitVec::new(0);
        for _ in 0..7 { bv.push_zero(); }
        for _ in 0..7 { bv.push_one(); }
        bv.reset();
        let res = BigUint::from_bytes_le(bv.reset().as_ref());
        assert_eq!(bv.last, 0);
        assert_eq!(bv.crsr, 0);
        assert_eq!(res.to_u64().unwrap(), 0);
    }

}
