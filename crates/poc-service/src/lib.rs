#![no_std]

pub fn add(a: impl Into<i64>, b: impl Into<i64>) -> impl Into<i64> {
    a.saturating_add(b)
}
