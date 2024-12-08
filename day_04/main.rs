pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)] // needed for tests
pub enum Niceness {
    Nice(u32),    // Tuple variant
    Naughty(u32), // Tuple variant
}

#[derive(Debug)]
pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        Kid {
            name,
            niceness: if Self::is_nice(good_deeds, bad_deeds) {
                Niceness::Nice(good_deeds)
            } else {
                Niceness::Naughty(bad_deeds)
            },
        }
    }

    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        good_deeds as f32 * GOOD_WEIGHT >= bad_deeds as f32 * BAD_WEIGHT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice() {
        assert_eq!(Kid::is_nice(10, 2), true);
        assert_eq!(Kid::is_nice(2, 10), false);
        assert_eq!(Kid::is_nice(0, 0), false);
    }

    #[test]
    fn test_new_kid() {
        let kid = Kid::new(String::from("Alice"), 10, 2);
        assert_eq!(kid.name, "Alice");
        assert_eq!(kid.niceness, Niceness::Nice(10));

        let kid = Kid::new(String::from("Bob"), 2, 10);
        assert_eq!(kid.name, "Bob");
        assert_eq!(kid.niceness, Niceness::Naughty(10));
    }
}

pub fn main() {
    let kids = vec![
        Kid::new(String::from("Alice"), 10, 2),
        Kid::new(String::from("Bob"), 2, 10),
    ];

    for kid in kids {
        match kid.niceness {
            Niceness::Nice(good_deeds) => {
                println!("{} is nice with {} good deeds!", kid.name, good_deeds);
            }
            Niceness::Naughty(bad_deeds) => {
                println!("{} is naughty with {} bad deeds!", kid.name, bad_deeds);
            }
        }
    }
}
