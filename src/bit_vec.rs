use std::cell::RefCell;

/// Utility for efficiently building integers bit by bit.
pub struct BitVec {
    init: RefCell<Vec<u8>>,
    last: u8,
    cursor: u8,
}

impl BitVec {

    pub fn new(cap: usize) -> BitVec {
        BitVec {
            init: RefCell::new(Vec::with_capacity((cap / 8) + 1)),
            last: 0,
            cursor: 0,
        }
    }

    pub fn push_zero(&mut self) {
        if self.cursor == 7 {
            self.init.borrow_mut().push(self.last);
            self.last = 0;
            self.cursor = 0;
        }
        else {
            self.cursor += 1;
        }
    }

    pub fn push_one(&mut self) {
        if self.cursor == 7 {
            self.last += 128;
            self.init.borrow_mut().push(self.last);
            self.last = 0;
            self.cursor = 0;
        }
        else {
            self.last += 1 << self.cursor;
            self.cursor += 1;
        }
    }

    pub fn len(&self) -> usize {
        self.init.borrow().len() * 8 + (self.cursor as usize)
    }

    pub fn reset(&mut self) -> Vec<u8> {
        let cap = self.init.borrow().capacity();
        let mut vec = self.init.replace(Vec::with_capacity(cap));
        vec.push(self.last);
        self.last = 0;
        self.cursor = 0;
        vec
    }

}
