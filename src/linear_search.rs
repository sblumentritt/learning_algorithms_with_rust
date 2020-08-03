#[allow(dead_code)]

// - choice for short lists
// - simple and requires minimal code to implement
// - traverses a list from start to end or till a match is found
fn linear_search<'a, T: 'a, I>(lookup: T, input: I) -> Option<usize>
where
    T: std::cmp::PartialEq,
    I: IntoIterator<Item = &'a T>,
{
    for (index, item) in input.into_iter().enumerate() {
        if *item == lookup {
            return Some(index);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    mod slice {
        use super::*;
        mod integer {
            use super::*;

            #[test]
            fn first_occurrence_returned() {
                let input = [3, 4, 67, 234, 3, 34, 234, 79, 0, -1, 2536];

                assert_eq!(linear_search(3, &input), Some(0));
                assert_eq!(linear_search(2536, &input), Some(10));
                assert_eq!(linear_search(0, &input), Some(8));
                assert_eq!(linear_search(234, input.iter()), Some(3));
                assert_eq!(linear_search(-1, input.iter()), Some(9));
            }

            #[test]
            fn none_returned_if_lookup_not_available() {
                let input = [8, 34, 12, 9];

                assert_eq!(linear_search(1, &input), None);
                assert_eq!(linear_search(242316, &input), None);
                assert_eq!(linear_search(5, input.iter()), None);
                assert_eq!(linear_search(-7, input.iter()), None);
            }

            #[test]
            fn none_returned_given_an_empty_slice() {
                let input = [];

                assert_eq!(linear_search(0, &input), None);
                assert_eq!(linear_search(0, input.iter()), None);
            }
        }

        mod character {
            use super::*;

            #[test]
            fn first_occurrence_returned() {
                let input = ['d', 'e', 'c', 's', 'e', 'x', 'y', 'y', 'g'];

                assert_eq!(linear_search('d', &input), Some(0));
                assert_eq!(linear_search('g', &input), Some(8));
                assert_eq!(linear_search('y', input.iter()), Some(6));
                assert_eq!(linear_search('e', input.iter()), Some(1));
            }

            #[test]
            fn none_returned_if_lookup_not_available() {
                let input = ['o', 'f', 't'];

                assert_eq!(linear_search('z', &input), None);
                assert_eq!(linear_search('a', &input), None);
                assert_eq!(linear_search(';', input.iter()), None);
                assert_eq!(linear_search('p', input.iter()), None);
            }

            #[test]
            fn none_returned_given_an_empty_slice() {
                let input = [];

                assert_eq!(linear_search('q', &input), None);
                assert_eq!(linear_search('q', input.iter()), None);
            }
        }
    }

    mod vec {
        use super::*;
        mod integer {
            use super::*;

            #[test]
            fn first_occurrence_returned() {
                let input = vec![3, 4, 67, 234, 3, 34, 234, 79, 0, -1, 2536];

                assert_eq!(linear_search(3, &input), Some(0));
                assert_eq!(linear_search(2536, &input), Some(10));
                assert_eq!(linear_search(0, &input), Some(8));
                assert_eq!(linear_search(234, input.iter()), Some(3));
                assert_eq!(linear_search(-1, input.iter()), Some(9));
            }

            #[test]
            fn none_returned_if_lookup_not_available() {
                let input = vec![8, 34, 12, 9];

                assert_eq!(linear_search(1, &input), None);
                assert_eq!(linear_search(242316, &input), None);
                assert_eq!(linear_search(5, input.iter()), None);
                assert_eq!(linear_search(-7, input.iter()), None);
            }

            #[test]
            fn none_returned_given_an_empty_vec() {
                let input = Vec::new();

                assert_eq!(linear_search(0, &input), None);
                assert_eq!(linear_search(0, input.iter()), None);
            }
        }

        mod character {
            use super::*;

            #[test]
            fn first_occurrence_returned() {
                let input = vec!['d', 'e', 'c', 's', 'e', 'x', 'y', 'y', 'g'];

                assert_eq!(linear_search('d', &input), Some(0));
                assert_eq!(linear_search('g', &input), Some(8));
                assert_eq!(linear_search('y', input.iter()), Some(6));
                assert_eq!(linear_search('e', input.iter()), Some(1));
            }

            #[test]
            fn none_returned_if_lookup_not_available() {
                let input = vec!['o', 'f', 't'];

                assert_eq!(linear_search('z', &input), None);
                assert_eq!(linear_search('a', &input), None);
                assert_eq!(linear_search(';', input.iter()), None);
                assert_eq!(linear_search('p', input.iter()), None);
            }

            #[test]
            fn none_returned_given_an_empty_vec() {
                let input = Vec::new();

                assert_eq!(linear_search('q', &input), None);
                assert_eq!(linear_search('q', input.iter()), None);
            }
        }
    }
}
