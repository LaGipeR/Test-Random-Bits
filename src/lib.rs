pub struct RandomBits {
    bits: Vec<u32>,
}

impl RandomBits {
    const BITS_COUNT: usize = 20_000;
    pub fn new() -> RandomBits {
        let mut bits = Vec::with_capacity(Self::BITS_COUNT);
        let mut seed = 871246u64;
        for _ in 0..(Self::BITS_COUNT >> 5) {
            bits.push(Self::get_next_random_block(&mut seed));
        }

        RandomBits { bits }
    }

    fn get_next_random_block(x: &mut u64) -> u32 {
        let a = 134775813u64;
        let c = 1u64;

        let mut block = 0;

        for _ in 0..4 {
            *x = a * (*x) + c;
            *x &= u32::MAX as u64;

            block <<= 8;
            let b = ((*x) >> 16) as u32;
            block |= (b >> 8) ^ (b & ((1 << 8) - 1));
        }

        block
    }

    #[inline]
    fn get_bit(&self, idx: usize) -> bool {
        ((self.bits[idx >> 5] >> (idx & 0b11111)) & 1) == 1
    }

    const LOWER_BOUND_MONO_BIT_TEST: u32 = 9654;
    const UPPER_BOUND_MONO_BIT_TEST: u32 = 10346;
    pub fn mono_bit_test(&self) -> bool {
        let mut one_count = 0;

        for bits_block in self.bits.iter() {
            one_count += bits_block.count_ones();
        }

        Self::LOWER_BOUND_MONO_BIT_TEST <= one_count && one_count <= Self::UPPER_BOUND_MONO_BIT_TEST
    }

    const MAX_POSSIBLE_SEQUENCE_LEN: usize = 36;
    pub fn max_sequence_len_test(&self) -> bool {
        use std::cmp::max;

        let mut one_sequence_len = 0usize;
        let mut zero_sequence_len = 0usize;
        let mut max_sequence_len = 0usize;

        for i in 0..Self::BITS_COUNT {
            let bit = self.get_bit(i);
            if bit {
                one_sequence_len += 1;
                max_sequence_len = max(max_sequence_len, zero_sequence_len);
                zero_sequence_len = 0;
            } else {
                zero_sequence_len += 1;
                max_sequence_len = max(max_sequence_len, one_sequence_len);
                one_sequence_len = 0;
            }
        }
        max_sequence_len = max(max_sequence_len, max(one_sequence_len, zero_sequence_len));

        max_sequence_len <= Self::MAX_POSSIBLE_SEQUENCE_LEN
    }

    const M: usize = 4;
    const LOWER_BOUND_XI3: f64 = 1.03;
    const UPPER_BOUND_XI3: f64 = 57.4;
    pub fn pokker_test(&self) -> bool {
        assert_eq!(Self::BITS_COUNT % Self::M, 0);

        let k = Self::BITS_COUNT / Self::M;
        let mut sequence_count = vec![0u64; 1 << Self::M];

        let mut cur_pokker_block = 0usize;
        let mut cur_pokker_block_len = 0usize;
        for i in 0..Self::BITS_COUNT {
            let bit = self.get_bit(i);

            cur_pokker_block = (cur_pokker_block << 1) | bit as usize;
            cur_pokker_block_len += 1;

            if cur_pokker_block_len == Self::M {
                sequence_count[cur_pokker_block] += 1;
                cur_pokker_block = 0;
                cur_pokker_block_len = 0;
            }
        }

        let mut xi3 = (1 << Self::M) as f64 / k as f64;
        let mut sum = 0u64;
        for n_i in sequence_count {
            sum += n_i * n_i;
        }

        xi3 *= sum as f64;
        xi3 -= k as f64;

        Self::LOWER_BOUND_XI3 <= xi3 && xi3 <= Self::UPPER_BOUND_XI3
    }

    const SEQUENCE_LEN_COUNT_SIZE: usize = 6;
    const LOWER_SEQUENCE_LEN_COUNT: [usize; Self::SEQUENCE_LEN_COUNT_SIZE] =
        [2267, 1079, 502, 223, 90, 90];
    const UPPER_SEQUENCE_LEN_COUNT: [usize; Self::SEQUENCE_LEN_COUNT_SIZE] =
        [2733, 1421, 748, 402, 223, 223];
    pub fn sequence_len_test(&self) -> bool {
        let mut one_sequence_len = 0usize;
        let mut sequence_len_count = vec![0usize; Self::LOWER_SEQUENCE_LEN_COUNT.len()];

        for i in 0..Self::BITS_COUNT {
            let bit = self.get_bit(i);
            if bit {
                one_sequence_len += 1;
            } else {
                if one_sequence_len != 0 {
                    if one_sequence_len >= sequence_len_count.len() {
                        sequence_len_count[Self::SEQUENCE_LEN_COUNT_SIZE - 1] += 1;
                    } else {
                        sequence_len_count[one_sequence_len - 1] += 1;
                    }
                    one_sequence_len = 0;
                }
            }
        }

        for i in 0..sequence_len_count.len() {
            if !(Self::LOWER_SEQUENCE_LEN_COUNT[i] <= sequence_len_count[i]
                && sequence_len_count[i] <= Self::UPPER_SEQUENCE_LEN_COUNT[i])
            {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let rnd_bits = RandomBits::new();

        assert!(rnd_bits.mono_bit_test());
        assert!(rnd_bits.max_sequence_len_test());
        assert!(rnd_bits.pokker_test());
        assert!(rnd_bits.sequence_len_test());
    }
}
