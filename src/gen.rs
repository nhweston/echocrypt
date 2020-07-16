use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::cast::ToPrimitive;

use crate::bit_vec::BitVec;

pub fn generate(
    data: &Vec<u8>,
    charset: &Vec<u8>,
    len: usize,
) -> Vec<u8> {
    let val = hash(data, charset.len(), len);
    decode(val, charset, len)
}

pub fn hash(
    data: &Vec<u8>,
    base: usize,
    len: usize
) -> BigUint {
    let mut bvec = BitVec::new();
    let num_blocks = BigUint::from(base).pow(len as u32).bits();
    let block_len = data.len() / (num_blocks as usize);
    let mut i = num_blocks;
    let mut idx = 0;
    loop {
        let idx_next = idx + block_len;
        if hash_block(data, idx, idx_next) {
            bvec.push_one();
        }
        else {
            bvec.push_zero();
        }
        i -= 1;
        if i == 0 {
            break;
        }
        idx = idx_next;
    }
    BigUint::from_bytes_le(bvec.result().as_ref())
}

pub fn hash_block(
    data: &Vec<u8>,
    start: usize,
    end: usize
) -> bool {
    let mut result = 0u8;
    for idx in start..end {
        result ^= data[idx];
    }
    result & 1 != 0
}

pub fn decode(
    value: BigUint,
    charset: &Vec<u8>,
    len: usize
) -> Vec<u8> {
    let base = charset.len();
    let mut result = Vec::with_capacity(len);
    let mut value = value;
    for _ in 0..len {
        let (quo, rem) = value.div_mod_floor(&base.into());
        value = quo;
        let code: usize = rem.to_usize().unwrap();
        let char = charset[code];
        result.push(char);
    }
    result
}
