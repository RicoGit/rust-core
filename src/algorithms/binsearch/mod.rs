use self::SearchResult::*;

#[derive(Debug, PartialOrd, PartialEq, Eq)]
pub enum SearchResult {
    Found(usize),
    InsertionPoint(usize),
}

pub fn binary_search<T: PartialOrd>(arr: &[T], key: T) -> SearchResult {
    search(arr, 0, arr.len(), key)
}

//
// Private
//

fn search<T: PartialOrd>(arr: &[T], from: usize, to: usize, key: T) -> SearchResult {
    if arr.is_empty() {
        return InsertionPoint(0);
    }

    if from == to {
        return InsertionPoint(from);
    }

    let mid = from + (to - from) / 2;

    if arr[mid] == key {
        return Found(mid);
    }

    if key > arr[mid] {
        return search(arr, mid + 1, to, key);
    } else {
        return search(arr, from, mid, key);
    };
}

#[cfg(test)]
mod tests {

    use super::binary_search;
    use super::SearchResult::*;

    #[test]
    fn binary_search_test() {
        let arr = [1, 3, 5, 7, 8, 8, 12, 13, 15];

        assert_eq!(InsertionPoint(0), binary_search(&[], 0));
        assert_eq!(InsertionPoint(0), binary_search(&arr, 0));
        assert_eq!(InsertionPoint(6), binary_search(&arr, 10));
        assert_eq!(InsertionPoint(arr.len()), binary_search(&arr, 20));
        assert_eq!(Found(0), binary_search(&arr, 1));
        assert_eq!(Found(3), binary_search(&arr, 7));
        assert_eq!(Found(4), binary_search(&arr, 8));
        assert_eq!(Found(arr.len() - 1), binary_search(&arr, 15));
    }

}
