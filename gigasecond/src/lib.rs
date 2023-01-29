use time::{Duration, PrimitiveDateTime as DateTime};
use core::ops::{Add};

pub fn after(start: DateTime) -> DateTime {
    start.add(Duration::seconds(1_000_000_000))
}
