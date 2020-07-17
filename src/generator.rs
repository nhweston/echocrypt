use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::cast::ToPrimitive;

use crate::bit_vec::BitVec;

pub struct Generator {

    /// The character set.
    cset: Vec<u8>,

    /// The buffer for the current password.
    bf: BitVec,

    /// The length of each block.
    blk_len: usize,

    /// The length of each password in characters.
    pwd_len: usize,

    /// The number of blocks for each password.
    num_blks: usize,

    /// The value of the current block.
    blk: u8,

    /// The number of bytes left to accumulate before committing the current block to the buffer.
    blk_left: usize,

}

impl Generator {

    pub fn new(
        cset: Vec<u8>,
        blk_len: usize,
        pwd_len: usize,
    ) -> Generator {
        let base = cset.len();
        let num_blks = BigUint::from(base).pow(pwd_len as u32).bits() as usize;
        let bf = Vec::with_capacity(num_blks);
        Generator {
            cset,
            bf: BitVec::new(num_blks),
            blk_len,
            pwd_len,
            num_blks,
            blk: 0,
            blk_left: blk_len,
        }
    }

    /// Pushes bytes to this generator. Returns passwords generated (if any) by this operation.
    pub fn push(&mut self, bytes: Vec<u8>) -> Vec<Vec<u8>> {
        let mut res = Vec::new();
        let mut blk = self.blk;
        if bytes.len() < self.blk_left {
            // bytes will not fill the current block
            for byte in bytes {
                blk ^= byte;
            }
            self.blk = 0;
            self.blk_left -= bytes.len();
        }
        else {
            // fill the current block
            let fst_left = self.blk_left;
            for idx in 0..fst_left {
                blk ^= bytes[idx];
            }
            if let Some(pwd) = self.commit(blk) {
                res.push(pwd);
            }
            // fill as many entire blocks as possible
            self.blk_left = self.blk_len;
            let num_full_blks = (bytes - fst_left) / self.blk_len;
            let mut idx = fst_left;
            for i_blk in 0..num_full_blks {
                blk = 0;
                for _ in 0..self.blk_len {
                    blk ^= bytes[idx];
                    idx += 1;
                }
                if let Some(pwd) = self.commit(blk) {
                    res.push(pwd);
                }
            }
            // push remaining bytes
            blk = 0;
            self.blk_left = self.blk_len + idx - bytes.len();
            for _ in idx..bytes.len() {
                blk ^= bytes[idx];
            }
            self.blk = blk;
        }
        res
    }

    /// Appends the current block to the buffer. If a password is completed by this operation, it
    /// is returned.
    fn commit(&mut self, blk: u8) -> Option<Vec<u8>> {
        if blk & 1 == 0 {
            self.bf.push_zero();
        }
        else {
            self.bf.push_one();
        }
        if self.bf.len() > self.num_blks {
            let val = BigUint::from_bytes_le(self.bf.result().as_ref());
            Some(self.decode(val))
        }
        else { None }
    }

    /// Decodes the given integer into a password.
    fn decode(&self, val: BigUint) -> Vec<u8> {
        let base = self.cset.len();
        let mut result = Vec::with_capacity(self.pwd_len);
        let mut val = val;
        for _ in 0..self.pwd_len {
            let (quo, rem) = val.div_mod_floor(&base.into());
            val = quo;
            let code: usize = rem.to_usize().unwrap();
            let char = self.cset[code];
            result.push(char);
        }
        result
    }

}
