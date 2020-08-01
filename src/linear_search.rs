#[allow(dead_code)]

// - choice for short lists
// - simple and requires minimal code to implement
// - traverses a list from start to end or till a match is found
fn linear_search<T>(lookup: T, list: &[T]) -> Option<usize>
where
    T: std::cmp::PartialEq,
{
    for (index, item) in list.iter().enumerate() {
        if *item == lookup {
            return Some(index);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    mod integer {
        use super::*;

        #[test]
        fn first_occurrence_returned() {
            let input = [3, 4, 67, 234, 3, 34, 234, 79, 0, -1, 2536];

            assert_eq!(linear_search(3, &input), Some(0));
            assert_eq!(linear_search(2536, &input), Some(10));
            assert_eq!(linear_search(0, &input), Some(8));
            assert_eq!(linear_search(234, &input), Some(3));
            assert_eq!(linear_search(-1, &input), Some(9));
        }

        #[test]
        fn none_returned_if_lookup_not_in_list() {
            let input = [3, 4, 67, 234, 3, 34, 234, 79, 0, -1, 2536];

            assert_eq!(linear_search(1, &input), None);
            assert_eq!(linear_search(242316, &input), None);
            assert_eq!(linear_search(5, &input), None);
            assert_eq!(linear_search(-7, &input), None);
        }

        #[test]
        fn none_returned_given_an_empty_list() {
            let input = [];

            assert_eq!(linear_search(0, &input), None);
        }
    }

    mod character {
        use super::*;

        #[test]
        fn first_occurrence_returned() {
            let input = ['d', 'e', 'c', 's', 'e', 'x', 'y', 'y', 'g'];

            assert_eq!(linear_search('d', &input), Some(0));
            assert_eq!(linear_search('g', &input), Some(8));
            assert_eq!(linear_search('y', &input), Some(6));
            assert_eq!(linear_search('e', &input), Some(1));
        }

        #[test]
        fn none_returned_if_lookup_not_in_list() {
            let input = ['d', 'e', 'c', 's', 'e', 'x', 'y', 'y', 'g'];

            assert_eq!(linear_search('z', &input), None);
            assert_eq!(linear_search('a', &input), None);
            assert_eq!(linear_search(';', &input), None);
            assert_eq!(linear_search('p', &input), None);
        }

        #[test]
        fn none_returned_given_an_empty_list() {
            let input = [];

            assert_eq!(linear_search('q', &input), None);
        }
    }
}
