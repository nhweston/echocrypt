pub struct BitVec {
    init: Vec<u8>,
    last: u8,
    len: usize,
    crsr: u8
}

impl BitVec {

    pub fn new() -> BitVec {
        BitVec {
            init: Vec::new(),
            last: 0,
            len: 1,
            crsr: 0
        }
    }

    pub fn push_zero(&mut self) {
        if self.crsr == 7 {
            self.init.push(self.last);
            self.last = 0;
            self.len += 1;
            self.crsr = 0;
        }
        else {
            self.last <<= 1;
            self.crsr += 1;
        }
    }

    pub fn push_one(&mut self) {
        if self.crsr == 7 {
            self.init.push(self.last);
            self.last = 1;
            self.len += 1;
            self.crsr = 0;
        }
        else {
            self.last = (self.last << 1) + 1;
            self.crsr += 1;
        }
    }

    pub fn result(self) -> Vec<u8> {
        let mut vec = self.init;
        vec.push(self.last);
        vec
    }

}
