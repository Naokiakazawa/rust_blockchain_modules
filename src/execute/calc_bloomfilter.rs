use modules::bloomfilter::BloomFilter;
use tracing::info;

pub fn bloomfilter() {
    let items = [
        "apple", "orange", "banana", "apricot", "avocado", "mango", "peach", "pear",
    ];
    let fp_rate = 0.001;
    let mut bf: BloomFilter = BloomFilter::new(items.len(), fp_rate);

    for item in &items {
        bf.set(item);
    }

    for item in &items {
        assert!(bf.check(item));
    }

    let (bitvec, k, m, n) = bf.get_summary();
    info!(
        "bitvec: {:?}, hash_count: {}, size: {}, items: {}",
        bitvec, k, m, n
    );
}
