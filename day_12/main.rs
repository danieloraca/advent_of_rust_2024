use std::{cmp::Ordering, error::Error, ops::Deref};

pub fn find_most_dense_location(locations: &[Location]) -> Result<&Location, Box<dyn Error>> {
    locations
        .iter()
        .max_by(|a, b| {
            a.density()
                .partial_cmp(&b.density())
                .unwrap_or(Ordering::Equal)
        })
        .ok_or("No locations found".into())
}

pub fn find_nearest_location(locations: &[Location]) -> Result<&Location, Box<dyn Error>> {
    locations
        .iter()
        .filter(|location| location.density() >= 1000.0)
        .min_by(|a, b| {
            a.x.hypot(a.y)
                .partial_cmp(&b.x.hypot(b.y))
                .unwrap_or(Ordering::Equal)
        })
        .ok_or_else(|| "No locations found".into())
}

const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug, Clone, PartialEq)]
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
    fn from(kg: SnowKg) -> Self {
        let snowballs = (*kg / SNOWBALL_WEIGHT_KG).round() as i64;
        Snowball(snowballs)
    }
}
impl From<SnowLb> for Snowball {
    fn from(lb: SnowLb) -> Self {
        let snowballs = (*lb / SNOWBALL_WEIGHT_LB).round() as i64;
        Snowball(snowballs)
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub area: f64,
    pub snow: Snowball,
}
impl Location {
    pub fn new(x: f64, y: f64, z: f64, area: f64, snow: impl Into<Snowball>) -> Self {
        Self {
            x,
            y,
            z,
            area,
            snow: snow.into(),
        }
    }
    pub fn density(&self) -> f64 {
        if self.area > 0.0 {
            *self.snow as f64 / self.area
        } else {
            0.0
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let loc1 = Location::new(0.0, 2.0, 0.0, 50.0, SnowKg::new(10.0)); // Density: ~100 snowballs / 50 = 2
    let loc2 = Location::new(3.0, 4.0, 0.0, 25.0, SnowKg::new(100.0)); // Density: 500 snowballs / 25 = 20
    let loc3 = Location::new(0.5, 0.5, 0.0, 1.0, SnowKg::new(1000.0)); // Density: 5000 snowballs / 1 = 5000

    let locations = vec![loc1, loc2, loc3];

    match find_nearest_location(&locations) {
        Ok(location) => println!("Nearest location with sufficient density: {:?}", location),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}
