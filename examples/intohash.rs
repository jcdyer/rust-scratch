use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use std::iter::FromIterator;

fn main() {
    let pair_vec = vec![
        (1, 2),
        (3, 4),
        (5u8, 6u16),
    ];
    let hm: HashMap<u8, u16, RandomState> = HashMap::from_iter(pair_vec.into_iter());
    println!("{}", hm.get(&5).unwrap());
}
