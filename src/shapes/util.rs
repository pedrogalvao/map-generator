use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn pseudo_random_usize(seed: u32) -> usize {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    let hash: u64 = hasher.finish();
    let random_usize = hash as usize;
    return random_usize;
}
