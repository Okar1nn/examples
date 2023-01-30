pub fn factorial(num: u64) -> u64 {
    // // for const fn
    // let mut ret = 1;
    // let mut i = 2;
    // while i <= num {
    //     ret *= i;
    //     i += 1;
    // }
    // ret

    (2..=num).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
