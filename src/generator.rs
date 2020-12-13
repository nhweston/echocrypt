use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::cast::ToPrimitive;

use crate::bit_vec::BitVec;

pub struct Generator {

    /// The character set.
    charset: Vec<u8>,

    /// The buffer for the current password.
    buffer: BitVec,

    /// The length of each password in characters.
    pwd_len: usize,

    /// The number of samples for each password.
    num_samples: usize,

}

impl Generator {

    pub fn new(charset: Vec<u8>, pwd_len: usize) -> Generator {
        let base = charset.len();
        let num_samples = BigUint::from(base).pow(pwd_len as u32).bits() as usize;
        Generator {
            charset,
            buffer: BitVec::new(num_samples),
            pwd_len,
            num_samples,
        }
    }

    /// Pushes bits to this generator. Returns passwords generated (if any) by this operation.
    pub fn push(&mut self, bits: &Vec<bool>) -> Vec<Vec<u8>> {
        let mut result = Vec::new();
        for bit in bits {
            if *bit {
                self.buffer.push_one();
            }
            else {
                self.buffer.push_zero();
            }
            if self.buffer.len() >= self.num_samples {
                let value = BigUint::from_bytes_le(self.buffer.reset().as_ref());
                result.push(self.decode(value));
            }
        }
        result
    }

    /// Decodes the given integer into a password.
    fn decode(&self, val: BigUint) -> Vec<u8> {
        let base = self.charset.len();
        let mut result = Vec::with_capacity(self.pwd_len);
        let mut val = val;
        for _ in 0..self.pwd_len {
            let (quo, rem) = val.div_mod_floor(&base.into());
            val = quo;
            let code: usize = rem.to_usize().unwrap();
            let char = self.charset[code];
            result.push(char);
        }
        result
    }

}
