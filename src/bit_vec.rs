/// Utilities for efficiently building integers by
pub struct BitVec {
    init: Vec<u8>,
    last: u8,
    crsr: u8
}

impl BitVec {

    pub fn new(cap: usize) -> BitVec {
        BitVec {
            init: Vec::with_capacity(((cap - 1) / 8) + 1),
            last: 0,
            crsr: 0
        }
    }

    pub fn push_zero(&mut self) {
        if self.crsr == 7 {
            self.init.push(self.last);
            self.last = 0;
            self.crsr = 0;
        }
        else {
            self.crsr += 1;
            self.last += (1 << self.crsr);
        }
    }

    pub fn push_one(&mut self) {
        if self.crsr == 7 {
            self.init.push(self.last);
            self.last = 1;
            self.crsr = 0;
        }
        else {
            self.crsr += 1;
            self.last += (1 << self.crsr);
        }
    }

    pub fn len(&self) -> usize {
        self.init.len() * 8 + (self.crsr as usize)
    }

    pub fn result(self) -> Vec<u8> {
        let mut vec = self.init;
        vec.push(self.last);
        vec
    }

}
