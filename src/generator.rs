use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::cast::ToPrimitive;

use crate::bit_vec::BitVec;

pub struct Generator {

    /// The character set.
    cset: Vec<u8>,

    /// The buffer for the current password.
    bf: BitVec,

    /// The length of each password in characters.
    pwd_len: usize,

    /// The number of samples for each password.
    num_smps: usize,

}

impl Generator {

    pub fn new(cset: Vec<u8>, pwd_len: usize) -> Generator {
        let base = cset.len();
        let num_smps = BigUint::from(base).pow(pwd_len as u32).bits() as usize;
        Generator {
            cset,
            bf: BitVec::new(num_smps),
            pwd_len,
            num_smps,
        }
    }

    /// Pushes bits to this generator. Returns passwords generated (if any) by this operation.
    pub fn push(&mut self, bits: &Vec<bool>) -> Vec<Vec<u8>> {
        let mut res = Vec::new();
        for bit in bits {
            if *bit {
                self.bf.push_one();
            }
            else {
                self.bf.push_zero();
            }
            if self.bf.len() >= self.num_smps {
                let val = BigUint::from_bytes_le(self.bf.reset().as_ref());
                res.push(self.decode(val));
            }
        }
        res
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
