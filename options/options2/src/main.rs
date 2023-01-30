#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if let statement whose value is "Some" type
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let mut range = 10;
        let mut optional_integers: Vec<Option<i8>> = Vec::new();
        for i in 0..=range {
            optional_integers.push(Some(i));
        }
        while let Some(integer) = optional_integers.pop() {
            if let Some(i) = integer {
                assert_eq!(i, range);
            }
            range -= 1;
        }
    }
}
