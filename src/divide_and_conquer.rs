use std::cmp::Ordering;

fn sum<T>(list: &[T]) -> T
where
    T: Default + Clone + std::ops::Add<Output = T>,
{
    match list.split_first() {
        Some((first, rest)) => first.clone() + sum(rest),
        None => T::default(),
    }
}

fn count_rec<T: Clone>(list: &[T]) -> i32 {
    match list.split_first() {
        Some((_, rest)) => 1 + count_rec(rest),
        None => 0,
    }
}

fn max_rec<T: Ord + Default + Clone + Copy>(list: &[T]) -> T {
    match list.split_first() {
        Some((first, rest)) => match rest.split_first() {
            Some(_) => first.clone().max(max_rec(rest)),
            None => first.clone(),
        },
        None => T::default(),
    }
}

fn recursive_binary_search<T: Ord + Clone>(list: &[T], item: T) -> Option<usize> {
    if list.is_empty() {
        return None;
    }
    let mid = list.len() >> 1;

    match list[mid].cmp(&item) {
        Ordering::Equal => Some(mid),
        Ordering::Less => {
            recursive_binary_search(&list[mid + 1..], item).map(|index| index + mid + 1)
        }
        Ordering::Greater => recursive_binary_search(&list[..mid], item),
    }
}

fn quicksort<T: Ord + Clone + Copy>(list: &[T]) -> Vec<T> {
    if list.len() < 2 {
        return list.to_vec();
    }

    let pivot = list[0];
    let (less, greater): (Vec<T>, Vec<T>) = list[1..].iter().partition(|x| **x < pivot);

    [quicksort(&less), vec![pivot.clone()], quicksort(&greater)].concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(10, sum(&[1, 2, 3, 4]));
        assert_eq!(0, sum(&[]));
    }

    #[test]
    fn test_count_rec() {
        assert_eq!(3, count_rec(&[1, 2, 3]));
    }

    #[test]
    fn test_max_rec() {
        assert_eq!(10, max_rec(&[2, 4, 6, 10, 8]));
        assert_eq!(100, max_rec(&[100, 4, 6, 10, 8]));
    }

    #[test]
    fn test_recursive_binary_search() {
        assert_ne!(
            None,
            recursive_binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5)
        );
        assert_eq!(
            None,
            recursive_binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 105)
        );
    }

    #[test]
    fn test_quicksort() {
        assert_eq!(quicksort(&[5, 3, 6, 2, 10]), &[2, 3, 5, 6, 10]);
        assert_eq!(
            quicksort(&[50, 3, 33, 87, 6, 2, 100]),
            &[2, 3, 6, 33, 50, 87, 100]
        );
    }
}
