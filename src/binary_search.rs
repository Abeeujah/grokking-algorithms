use std::cmp::Ordering;

pub fn binary_search<T>(list: &[T], item: &T) -> Option<usize>
where
    T: Ord,
{
    // Start searching from the middle
    // If the item at the middle is
    // the item you're looking for,
    // return the middle index
    // If the item at the middle is greater
    // than the item you're searching for,
    // Set search scope upper bound to be mid - 1
    // Similarly, if the item at the middle is
    // less than the item you're searching for,
    // Set search scope lower bound to be mid + 1
    let mut lower_bound = 0;
    let mut upper_bound = list.len() - 1;

    while lower_bound <= upper_bound {
        let mid = (lower_bound + upper_bound) >> 1;
        let guess = &list[mid];
        match guess.cmp(&item) {
            Ordering::Less => lower_bound = mid + 1,
            Ordering::Greater => upper_bound = mid - 1,
            Ordering::Equal => return Some(mid),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(Some(4), binary_search(&list[..], &5));
        assert_eq!(None, binary_search(&list[..], &12));
        assert_eq!(Some(0), binary_search(&list[..], &1));
        assert_eq!(Some(9), binary_search(&list[..], &10));
    }
}
