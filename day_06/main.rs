// If s1 is longer than s2, return a reference to s1 otherwise return a reference to s2 inside a Some variant.
// If both slices have the same length, return None.
// Any white spaces in the beginning or end of the string should be trimmed.

// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let s1_trimmed = s1.trim();
    let s2_trimmed = s2.trim();

    match s1_trimmed.chars().count() as i32 - s2_trimmed.chars().count() as i32 {
        n if n > 0 => Some(s1_trimmed),
        n if n < 0 => Some(s2_trimmed),
        _ => None,
    }
}

/*
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    // Your code here
    match s1.trim().chars().count() as i32 - s2.trim().chars().count() as i32{
        0 => None,
        x if x < 0 => Some(s2),
        _ => Some(s1)
    }
}
*/
