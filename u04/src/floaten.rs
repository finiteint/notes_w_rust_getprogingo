#[derive(Debug, Clone, Copy, Default)]
pub struct Hf64(f64);

impl Hf64 {
    pub fn new(f: f64) -> Self {
        Self(f)
    }

    pub fn as_f64(self) -> f64 {
        self.0
    }

    pub fn update<F>(&mut self, f: F)
    where
        F: Fn(f64) -> f64,
    {
        self.0 = f(self.0)
    }
}

impl std::cmp::Eq for Hf64 {}

impl std::cmp::PartialEq for Hf64 {
    fn eq(&self, other: &Self) -> bool {
        if self.0.is_nan() {
            other.0.is_nan()
        } else {
            self.0.eq(&other.0)
        }
    }
}

impl std::hash::Hash for Hf64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.0.to_bits())
    }
}

impl std::cmp::PartialOrd for Hf64 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use std::cmp::Ordering::*;
        if self.0.is_nan() {
            if other.0.is_nan() {
                Some(Equal)
            } else {
                Some(Less)
            }
        } else if other.0.is_nan() {
            Some(Greater)
        } else {
            self.0.partial_cmp(&other.0)
        }
    }
}

impl std::cmp::Ord for Hf64 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Less)
    }
}

impl std::fmt::Display for Hf64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
