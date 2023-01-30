use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.values().filter(|val| val == &&value).count()
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection
        .iter()
        .map(|map| count_iterator(map, value))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(3, count_iterator(&map, Progress::Complete));
    }

    #[test]
    fn count_equals_for() {
        let map = get_map();
        assert_eq!(
            count_for(&map, Progress::Complete),
            count_iterator(&map, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            6,
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_equals_for() {
        let collection = get_vec_map();
        assert_eq!(
            count_collection_for(&collection, Progress::Complete),
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    fn get_map() -> HashMap<String, Progress> {
        use Progress::{Complete, None, Some};

        let mut map = HashMap::new();
        map.insert("variables1".to_owned(), Complete);
        map.insert("functions1".to_owned(), Complete);
        map.insert("hashmap1".to_owned(), Complete);
        map.insert("arc1".to_owned(), Some);
        map.insert("as_ref_mut".to_owned(), None);
        map.insert("from_str".to_owned(), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::{Complete, None};

        let map = get_map();

        let mut other = HashMap::new();
        other.insert("variables2".to_owned(), Complete);
        other.insert("functions2".to_owned(), Complete);
        other.insert("if1".to_owned(), Complete);
        other.insert("from_into".to_owned(), None);
        other.insert("try_from_into".to_owned(), None);

        vec![map, other]
    }
}
