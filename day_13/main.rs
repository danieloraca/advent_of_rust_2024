// Import the necessary modules
#[derive(Debug)]
pub enum ParseError {
    NoName,
    NoGoodDeeds,
    NoBadDeeds,
    InvalidGoodDeeds,
    InvalidBadDeeds,
}
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoName => write!(f, "Name field is missing"),
            Self::NoGoodDeeds => write!(f, "Good deeds field is missing"),
            Self::NoBadDeeds => write!(f, "Bad deeds field is missing"),
            Self::InvalidGoodDeeds => write!(f, "Good deeds value is invalid"),
            Self::InvalidBadDeeds => write!(f, "Bad deeds value is invalid"),
        }
    }
}
impl std::error::Error for ParseError {}
// 2. Implement the Error trait for ParseError
#[derive(Debug)]
pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}
impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };
        Kid { name, niceness }
    }
    pub fn parse_row(csv_row: &str) -> Result<Kid, ParseError> {
        // 3. Update the code to return meaningful errors
        let mut fields = csv_row.split(',');
        let name = fields
            .next()
            .filter(|f| !f.is_empty())
            .ok_or(ParseError::NoName)?
            .to_string();
        let good_deeds = fields
            .next()
            .filter(|f| !f.is_empty())
            .ok_or(ParseError::NoGoodDeeds)?
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidGoodDeeds)?;

        let bad_deeds = fields
            .next()
            .filter(|f| !f.is_empty())
            .ok_or(ParseError::NoBadDeeds)?
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidBadDeeds)?;
        Ok(Kid::new(name, good_deeds, bad_deeds))
    }
    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }
        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;
        let ratio = good_deeds / (good_deeds + bad_deeds);
        ratio >= 0.75
    }
}
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;
#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}

fn main() {
    let csv_data = "Alice,0,0\nBob,1,2\nCharlie,3,1\n";
    let kids: Vec<Kid> = csv_data
        .lines()
        .map(|line| Kid::parse_row(line))
        .collect::<Result<Vec<Kid>, ParseError>>()
        .unwrap();
    println!("{:?}", kids);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_row() {
        let csv_row = "Alice,0,0";
        let kid = Kid::parse_row(csv_row).unwrap();
        assert_eq!(kid.name, "Alice");
        assert_eq!(kid.niceness, Niceness::Naughty);
    }

    #[test]
    fn test_is_nice() {
        assert_eq!(Kid::is_nice(0, 0), false);
        assert_eq!(Kid::is_nice(1, 2), false);
        assert_eq!(Kid::is_nice(3, 1), true);
    }

    #[test]
    fn test_parse_row_error() {
        let csv_row = "Alice,0,";
        let error = Kid::parse_row(csv_row).unwrap_err();
        assert_eq!(error, ParseError::NoBadDeeds);
    }

    #[test]
    fn test_parse_row_error_invalid() {
        let csv_row = "Alice,0,invalid";
        let error = Kid::parse_row(csv_row).unwrap_err();
        assert_eq!(error, ParseError::InvalidBadDeeds);
    }

    #[test]
    fn test_parse_row_error_no_name() {
        let csv_row = ",0,0";
        let error = Kid::parse_row(csv_row).unwrap_err();
        assert_eq!(error, ParseError::NoName);
    }
}
