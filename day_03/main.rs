pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    if good_deeds == 0 && bad_deeds == 0 {
        return false;
    }

    let total_deeds = (good_deeds as f32 * GOOD_WEIGHT) + (bad_deeds as f32 * BAD_WEIGHT);
    let nice_ratio = (good_deeds as f32 * GOOD_WEIGHT) / total_deeds;

    nice_ratio > 0.75
}

pub fn main() {
    let good_deeds = 10;
    let bad_deeds = 2;

    if is_nice(good_deeds, bad_deeds) {
        println!("This child is nice!");
    } else {
        println!("This child is naughty!");
    }
}
