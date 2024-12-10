use std::ops::Deref;

const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;

pub struct SnowKg(pub f64);

impl SnowKg {
    pub fn new(kg: f64) -> Self {
        SnowKg(kg)
    }
}

impl Deref for SnowKg {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct SnowLb(pub f64);

impl SnowLb {
    pub fn new(lb: f64) -> Self {
        SnowLb(lb)
    }
}

impl Deref for SnowLb {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Snowball(pub i64);

impl Snowball {
    pub fn new(snowballs: i64) -> Self {
        Snowball(snowballs)
    }
}

impl Deref for Snowball {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SnowKg> for Snowball {
    fn from(weight: SnowKg) -> Self {
        let num_snowballs = (weight.0 / SNOWBALL_WEIGHT_KG).round() as i64;
        Snowball::new(num_snowballs)
    }
}

impl From<SnowLb> for Snowball {
    fn from(weight: SnowLb) -> Self {
        let num_snowballs = (weight.0 / SNOWBALL_WEIGHT_LB).round() as i64;
        Snowball::new(num_snowballs)
    }
}

fn main() {
    let snow_weight_kg = SnowKg::new(1.0); // 1 kg of snow
    let snow_weight_lb = SnowLb::new(2.0); // 2 lbs of snow

    let snowballs_from_kg: Snowball = snow_weight_kg.into();
    let snowballs_from_lb: Snowball = snow_weight_lb.into();

    println!("Snowballs from 1 kg: {}", *snowballs_from_kg);
    println!("Snowballs from 2 lb: {}", *snowballs_from_lb);

    // Demonstrating dereferencing for SnowKg and SnowLb
    println!("Snow weight in kg: {}", *snow_weight_kg);
    println!("Snow weight in lb: {}", *snow_weight_lb);
}
