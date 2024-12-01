//! misc useful AoC functions

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Opaque type representing the hash of a value
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
pub struct HashVal(u64);

pub fn hash<T: Hash>(t: &T) -> HashVal {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    HashVal(s.finish())
}
