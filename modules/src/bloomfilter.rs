use bit_vec::BitVec;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

pub struct BloomFilter {
    pub bit_vec: BitVec,
    pub hash_count: u32,
    pub bitmap_size: usize,
    pub items_count: usize,
}

fn compute_bitmap_size(items_count: usize, fp_p: f64) -> usize {
    assert!(items_count > 0);
    assert!(fp_p > 0.0 && fp_p < 1.0);
    let log2: f64 = std::f64::consts::LN_2;
    let log2_2: f64 = log2 * log2;
    ((items_count as f64) * f64::ln(fp_p) / (-8.0 * log2_2)).ceil() as usize
}

fn compute_hash_count(items_count: usize, bitmap_size: usize) -> u32 {
    assert!(items_count > 0);
    assert!(bitmap_size > 0);
    let log2: f64 = std::f64::consts::LN_2;
    ((bitmap_size as f64 / items_count as f64) * log2).ceil() as u32
}

impl BloomFilter {
    pub fn new(items_count: usize, fp_rate: f64) -> Self {
        let bitmap_size: usize = compute_bitmap_size(items_count, fp_rate);
        let hash_count: u32 = compute_hash_count(items_count, bitmap_size);

        BloomFilter {
            bit_vec: BitVec::from_elem(bitmap_size, false),
            hash_count,
            bitmap_size,
            items_count,
        }
    }

    pub fn set(&mut self, item: &str) {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        hasher.write(item.as_bytes());
        let hash: u64 = hasher.finish();
        let first_half_hash: u32 = hash as u32;
        let second_half_hash: u32 = (hash >> 32) as u32;

        for i in 0..self.hash_count {
            let index = (first_half_hash.wrapping_add(i.wrapping_mul(second_half_hash)))
                % (self.bitmap_size as u32);
            self.bit_vec.set(index as usize, true);
        }
    }

    pub fn check(&self, item: &str) -> bool {
        let mut hasher: DefaultHasher = DefaultHasher::new();
        hasher.write(item.as_bytes());
        let hash: u64 = hasher.finish();
        let first_half_hash: u32 = hash as u32;
        let second_half_hash: u32 = (hash >> 32) as u32;

        for i in 0..self.hash_count {
            let index = (first_half_hash.wrapping_add(i.wrapping_mul(second_half_hash)))
                % (self.bitmap_size as u32);
            if !self.bit_vec.get(index as usize).unwrap() {
                return false;
            }
        }
        true
    }

    pub fn get_summary(&self) -> (&BitVec, u32, usize, usize) {
        (
            &self.bit_vec,
            self.hash_count,
            self.bitmap_size,
            self.items_count,
        )
    }
}
