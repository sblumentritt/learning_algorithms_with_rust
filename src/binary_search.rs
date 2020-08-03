#[allow(dead_code)]

// - process of searching for a specific value in an ordered collection
// - starts at the middle of the collection
// - if target number greater than middle search upper half
// - repeats this process until it finds the target
// - more complicated than linear search but faster for large data
fn binary_search(lookup: i32, list: &[i32]) -> Option<usize> {
    if list.len() == 0 {
        return None;
    }

    let mut left: usize = 0;
    let mut right: usize = list.len() - 1;

    while left <= right {
        let mid: usize = (left + right) / 2;
        if list[mid] < lookup {
            left = mid + 1;
        } else if list[mid] > lookup {
            // prevent subtract overflow when lookup is negativ and not in the input
            if right == 0 {
                break;
            }

            right = mid - 1;
        } else {
            return Some(mid);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_in_list() {
        let input = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18,
        ];

        // l = 0
        // r = 18
        // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]
        // m = 9 -> m < 17
        // ---
        // l = 10
        // r = 18
        // [10, 11, 12, 13, 14, 15, 16, 17, 18]
        // m = 14 -> m < 17
        // ---
        // l = 15
        // r = 18
        // [15, 16, 17, 18]
        // m = 16(.5) -> m < 17
        // ---
        // l = 17
        // r = 18
        // [17, 18]
        // m = 17(.5) -> m == 17
        assert_eq!(binary_search(17, &input), Some(17));

        assert_eq!(binary_search(0, &input), Some(0));
        assert_eq!(binary_search(3, &input), Some(3));
        assert_eq!(binary_search(18, &input), Some(18));
        assert_eq!(binary_search(9, &input), Some(9));
    }

    #[test]
    fn none_returned_if_lookup_not_in_list() {
        let input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        assert_eq!(binary_search(17, &input), None);
        assert_eq!(binary_search(3214, &input), None);
        assert_eq!(binary_search(1415, &input), None);
        assert_eq!(binary_search(-1, &input), None);
        assert_eq!(binary_search(-547, &input), None);
    }

    #[test]
    fn none_returned_given_an_empty_list() {
        let input = [];

        assert_eq!(binary_search(1, &input), None);
    }
}
