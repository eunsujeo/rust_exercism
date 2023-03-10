#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let cars_per_hour = speed as f64 * 221.0;
    match speed {
        1..=4 => cars_per_hour,
        5..=8 => (cars_per_hour * 0.9),
        9..=10 => (cars_per_hour * 0.77),
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    production_rate_per_hour(speed) as u32 / 60
}
