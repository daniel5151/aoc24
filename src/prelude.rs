//! Useful types and traits for solving AoC style questions.
//!
//! Rome wasn't built in a day, and neither was this prelude. Expect this to
//! keep growing as more questions get solved.

// useful std stuff
pub use core::fmt::Debug;
pub use core::fmt::Display;
pub use std::collections::*;
pub use std::io::prelude::*;
pub use std::str::FromStr;

// useful extern crates
pub use itertools::Itertools;
pub use itertools::MinMaxResult;

pub use crate::util::aoc;
pub use crate::util::ext_traits::*;
pub use crate::util::gcd_lcm::GcdLcm;
pub use crate::DynResult;

/// More compact than the default `{:#?}` output, while still printing each
/// (k, v) on a new line.
#[macro_export]
macro_rules! dbg_map {
    ($map:expr) => {
        for (k, v) in $map.iter().sorted_by(|(k1, _), (k2, _)| k1.cmp(k2)) {
            eprintln!("{:?}: {:?}", k, v);
        }
    };
}
