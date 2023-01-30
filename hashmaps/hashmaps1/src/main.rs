fn fruit_basket() -> HashMap<String, u32> {
    // let basket = HashMap::from_iter([
    //     ("banana".to_owned(), 2),
    //     ("apple".to_owned(), 2),
    //     ("pear".to_owned(), 2),
    // ]);

    let mut basket = HashMap::new();

    basket.insert("banana".to_owned(), 2);
    basket.insert("apple".to_owned(), 2);
    basket.insert("pear".to_owned(), 2);

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
