//已知布隆过滤器长度为 n ，在可容忍的误差率为 ε 的情况下，此时最佳的存储个数为 m: m = − nlnε / (ln2)2
//
// 而此时需要的哈希函数个数 k 为:
// k = − lnε / ln2 = log2ε
//
// 假如容忍的误差率 ε = 8% ，那么 k = 3，k 越大代表误差率越大。在不改变容错率的情
// 况下，可以组合迭代次数和两个基本哈希函数来模拟 k 个哈希函数。
// gi(x) = h1(x) + ih2(x)

use std::collections::hash_map::DefaultHasher;
use std::f64::consts::LN_2;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

#[allow(unused)]
struct BloomFilter<T: ?Sized> {
    bits: Vec<bool>,
    hash_fn_count: usize,
    hasher: [DefaultHasher; 2],
    _phantom: PhantomData<T>,
}

#[allow(unused)]
impl<T: ?Sized + Hash> BloomFilter<T> {
    pub fn new(cap: usize, err_rate: f64) -> Self {
        let ln2_pow = LN_2.powf(2f64);
        let bits_size = (-1f64 * cap as f64 * ln2_pow / err_rate.ln()) as usize;
        let hash_fn_count = (-1f64 * err_rate.log2()) as usize;
        let hasher = [DefaultHasher::new(), DefaultHasher::new()];
        BloomFilter {
            bits: vec![false; bits_size],
            hash_fn_count,
            hasher,
            _phantom: PhantomData,
        }
    }

    fn cal_hash(&self, value: &T) -> (u64, u64) {
        let [mut hash1, mut hash2] = self.hasher.clone();
        value.hash(&mut hash1);
        value.hash(&mut hash2);
        (hash1.finish(), hash2.finish())
    }

    fn get_index(&self, hash_tuple: (u64, u64), fn_i: usize) -> usize {
        (hash_tuple.0 as usize).wrapping_add(fn_i.wrapping_mul(hash_tuple.1 as usize))
            % self.bits.len()
    }

    pub fn insert(&mut self, value: &T) {
        let hash_tuple = self.cal_hash(value);

        for fn_i in 0..self.hash_fn_count {
            let index = self.get_index(hash_tuple, fn_i);
            self.bits[index] = true;
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        let hash_tuple = self.cal_hash(value);
        (0..self.hash_fn_count).all(|fn_i| self.bits[self.get_index(hash_tuple, fn_i)])
    }
}

#[cfg(test)]
mod test {
    use crate::bloom_filter::BloomFilter;
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};
    use std::collections::HashSet;

    #[test]
    fn test_insert_and_query() {
        let cap = 10_000_000;
        let mut filter = BloomFilter::new(2 * cap, 0.08);
        for _ in 0..cap {
            let rand_str: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect::<String>();
            filter.insert(&rand_str);
            assert!(filter.contains(&rand_str))
        }
    }

    #[test]
    fn test_hash_set_insert_and_query() {
        let cap = 10_000_000;
        let mut filter = HashSet::new();
        for _ in 0..cap {
            let rand_str: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect::<String>();
            filter.insert(rand_str.clone());
            filter.contains(&rand_str);
        }
    }
}
