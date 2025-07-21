use std::collections::HashMap;
use std::hash::Hash;

pub struct Counter<T: Hash + Eq> {
    values: HashMap<T, u64>,
}

impl<T: Hash + Eq> Counter<T> {
    /// 새 Counter를 만듭니다.
    pub fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// 지정된 값의 발생 횟수를 셉니다.
    pub fn count(&mut self, value: T) {
        if self.values.contains_key(&value) {
            *self.values.get_mut(&value).unwrap() += 1;
        } else {
            self.values.insert(value, 1);
        }
    }

    /// 지정된 값이 표시된 횟수를 반환합니다.
    pub fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}