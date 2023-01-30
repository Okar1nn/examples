const fn maybe_icecream(time_of_day: u8) -> Option<u8> {
    // if time_of_day < 22 {
    //     Some(5)
    // } else if time_of_day <= 24 {
    //     Some(0)
    // } else {
    //     None
    // }
    match time_of_day {
        0..=21 => Some(5),
        22..=24 => Some(0),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(10), Some(5));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(25), None);
    }

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get at the value contained in the Option?
        let icecreams = maybe_icecream(12).unwrap_or(0);
        assert_eq!(icecreams, 5);
    }
}
