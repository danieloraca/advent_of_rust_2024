use std::fmt;

// 1. Add the `is_wrapped` field to the gift structs
pub struct KidsGift {
    pub name: String,
    pub is_wrapped: bool,
}

pub struct ElvesGift {
    pub name: String,
    pub is_wrapped: bool,
}

pub struct ReindeerGift {
    pub name: String,
    pub is_wrapped: bool,
}

pub trait Gift // 2. Finish the trait definition //
{
    fn wrap(&mut self);
}

// 3. Update the function signature
pub fn prepare_gift<T: fmt::Display + Gift>(gift: &mut T) {
    println!("Preparing gift for {}", &gift);
    gift.wrap();
    println!("Gift wrapped for {}", &gift);
}

// 4. Implement the Gift trait to the gift structs

impl Gift for KidsGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }
}

impl Gift for ElvesGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }
}

impl Gift for ReindeerGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }
}

impl fmt::Display for KidsGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for ElvesGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for ReindeerGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn main() {
    let mut kids_gift = KidsGift {
        name: "toy car".to_string(),
        is_wrapped: false,
    };
    let mut elves_gift = ElvesGift {
        name: "vertical monitor".to_string(),
        is_wrapped: false,
    };
    let mut reindeer_gift = ReindeerGift {
        name: "carrot".to_string(),
        is_wrapped: false,
    };

    prepare_gift(&mut kids_gift);
    prepare_gift(&mut elves_gift);
    prepare_gift(&mut reindeer_gift);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kids_gift_wrap() {
        let mut gift = KidsGift {
            name: "toy car".to_string(),
            is_wrapped: false,
        };
        assert_eq!(gift.is_wrapped, false);
        gift.wrap();
        assert_eq!(gift.is_wrapped, true);
    }

    #[test]
    fn test_elves_gift_wrap() {
        let mut gift = ElvesGift {
            name: "vertical monitor".to_string(),
            is_wrapped: false,
        };
        assert_eq!(gift.is_wrapped, false);
        gift.wrap();
        assert_eq!(gift.is_wrapped, true);
    }

    #[test]
    fn test_reindeer_gift_wrap() {
        let mut gift = ReindeerGift {
            name: "carrot".to_string(),
            is_wrapped: false,
        };
        assert_eq!(gift.is_wrapped, false);
        gift.wrap();
        assert_eq!(gift.is_wrapped, true);
    }

    #[test]
    fn test_prepare_gift() {
        let mut kids_gift = KidsGift {
            name: "toy car".to_string(),
            is_wrapped: false,
        };
        let mut elves_gift = ElvesGift {
            name: "vertical monitor".to_string(),
            is_wrapped: false,
        };
        let mut reindeer_gift = ReindeerGift {
            name: "carrot".to_string(),
            is_wrapped: false,
        };

        // Capture stdout to check print statements
        let mut buffer = Vec::new();
        use std::io::Write;
        let _ = std::io::stdout().write(&mut buffer);

        prepare_gift(&mut kids_gift);
        assert_eq!(kids_gift.is_wrapped, true);

        prepare_gift(&mut elves_gift);
        assert_eq!(elves_gift.is_wrapped, true);

        prepare_gift(&mut reindeer_gift);
        assert_eq!(reindeer_gift.is_wrapped, true);
    }
}
