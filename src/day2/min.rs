use std::cmp::Ord;
use std::cmp::Ordering;

pub fn min<T: Ord>(v1: T, v2: T) -> T {
    match v1.cmp(&v2) {
        Ordering::Greater => v2,
        _ => v1,
    }
}