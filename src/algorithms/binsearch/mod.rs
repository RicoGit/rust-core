#![allow(dead_code)]
use self::SearchResult::*;

#[derive(Debug, PartialOrd, PartialEq, Eq)]
pub enum SearchResult {
    Found(usize),
    InsertionPoint(usize),
}

pub fn binary_search<T: PartialOrd>(arr: &[T], key: T) -> SearchResult {
    if arr.is_empty() {
        return InsertionPoint(0);
    }

    search(arr, 0, arr.len(), key)
}

pub fn binary_search_fro_rotated_arr<T: PartialOrd>(arr: &[T], key: T) -> SearchResult {
    if arr.is_empty() {
        return InsertionPoint(0);
    }

    let pivot = find_pivot(arr, 0, arr.len() - 1);

    if pivot < 0 {
        return binary_search(arr, key);
    }

    let pivot: usize = pivot as usize;

    if arr[pivot] == key {
        return Found(pivot);
    };

    if key >= arr[0] {
        return search(arr, 0, pivot + 1, key);
    } else {
        return search(arr, pivot + 1, arr.len() - 1, key);
    }
}

//
// Private
//

fn search<T: PartialOrd>(arr: &[T], from: usize, to: usize, key: T) -> SearchResult {
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

/// Returns pivot point for rotated array.
/// For example returns for array [3, 4, 5, 6, 1, 2] it returns 3 (index of 6).
fn find_pivot<T: PartialOrd>(arr: &[T], from: usize, to: usize) -> isize {
    if from > to {
        return -1;
    }

    if from == to {
        return from as isize;
    }

    let mid = from + (to - from) / 2;

    if arr[mid] > arr[from] {
        return find_pivot(arr, mid, to);
    } else {
        return find_pivot(arr, from, mid);
    }
}

#[cfg(test)]
mod tests {
    use super::binary_search;
    use super::binary_search_fro_rotated_arr;
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

    #[test]
    fn binary_search_for_rotated_arr_test() {
        let rotated_arr = [12, 13, 15, 1, 3, 5, 7, 8, 8];

        assert_eq!(InsertionPoint(0), binary_search_fro_rotated_arr(&[], 0));
        assert_eq!(
            InsertionPoint(3),
            binary_search_fro_rotated_arr(&rotated_arr, 0)
        );
        assert_eq!(
            InsertionPoint(rotated_arr.len() - 1),
            binary_search_fro_rotated_arr(&rotated_arr, 10)
        );
        assert_eq!(
            InsertionPoint(3),
            binary_search_fro_rotated_arr(&rotated_arr, 20)
        );
        assert_eq!(Found(3), binary_search_fro_rotated_arr(&rotated_arr, 1));
        assert_eq!(Found(6), binary_search_fro_rotated_arr(&rotated_arr, 7));
        assert_eq!(Found(7), binary_search_fro_rotated_arr(&rotated_arr, 8));
        assert_eq!(Found(2), binary_search_fro_rotated_arr(&rotated_arr, 15));

        let normal_arr = [1, 3, 5, 7, 8, 8, 12, 13, 15];

        assert_eq!(InsertionPoint(0), binary_search(&normal_arr, 0));
        assert_eq!(InsertionPoint(6), binary_search(&normal_arr, 10));
        assert_eq!(
            InsertionPoint(normal_arr.len()),
            binary_search(&normal_arr, 20)
        );
        assert_eq!(Found(0), binary_search(&normal_arr, 1));
        assert_eq!(Found(3), binary_search(&normal_arr, 7));
        assert_eq!(Found(4), binary_search(&normal_arr, 8));
        assert_eq!(Found(normal_arr.len() - 1), binary_search(&normal_arr, 15));
    }
}
