use modules::bloomfilter::BloomFilter;

pub fn bloomfilter() {
    let items = ["hoge", "fuga", "piyo"];
    let fp_rate = 0.01;
    let mut bf: BloomFilter = BloomFilter::new(items.len(), fp_rate);

    for item in &items {
        bf.set(item);
    }

    for item in &items {
        assert!(bf.check(item));
    }
}
