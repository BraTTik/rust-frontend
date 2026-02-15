use std::collections::BTreeMap;

pub fn closest_neighbors<'a, K, V>(map: &'a BTreeMap<K, V>, key: &K) -> (Option<(&'a K, &'a V)>, Option<(&'a K, &'a V)>)
where K: Ord
{
    let mut min: Option<(&'a K, &'a V)> = None;
    let mut max: Option<(&'a K, &'a V)> = None;

    for (k, v) in map.iter() {
        if k == key {
            min = Some((k, v));
            max = Some((k, v));
            break;
        }
        if k < key {
            min = Some((k, v));
        }
        if k > key {
            max = Some((k, v));
        }

        if (min.is_some() && max.is_some()) {
            break;
        }
    }

    (min, max)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut map = BTreeMap::new();
        map.insert(10, "A");
        map.insert(20, "B");
        map.insert(30, "C");
        map.insert(50, "D");

        assert_eq!(closest_neighbors(&map, &25), (Some((&20, &"B")), Some((&30, &"C"))));
    }

    #[test]
    fn test_exact_match() {
        let mut map = BTreeMap::new();
        map.insert(42, "answer");
        assert_eq!(closest_neighbors(&map, &42), (Some((&42, &"answer")), Some((&42, &"answer"))));
    }

    #[test]
    fn test_empty() {
        let map: BTreeMap<i32, &str> = BTreeMap::new();
        assert_eq!(closest_neighbors(&map, &42), (None, None));
    }

    #[test]
    fn test_above_all() {
        let mut map = BTreeMap::new();
        map.insert(100, "top");
        assert_eq!(closest_neighbors(&map, &42), (None, Some((&100, &"top"))));
    }

    #[test]
    fn test_below_all() {
        let mut map = BTreeMap::new();
        map.insert(1, "bottom");
        assert_eq!(closest_neighbors(&map, &42), (Some((&1, &"bottom")), None));
    }

    #[test]
    fn test_strings() {
        let mut map = BTreeMap::new();
        map.insert("apple", 1);
        map.insert("banana", 2);
        map.insert("cherry", 3);
        assert_eq!(closest_neighbors(&map, &"apricot"), (Some((&"apple", &1)), Some((&"banana", &2))));
    }
}