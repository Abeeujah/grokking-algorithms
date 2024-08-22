pub fn smallest<T: Ord + Copy>(list: &[T]) -> usize {
    // Return the smallest Item in a list.
    // Traverse the entire list to be sure.
    let mut smallest_index = 0;

    // Check the first item against every item in the list
    // Update the smallest index anytime you encounter an item
    // smaller than the previous one.
    for (index, element) in list.iter().enumerate().skip(1) {
        if element < &list[smallest_index] {
            smallest_index = index;
        }
    }

    // Return smallest index.
    smallest_index
}

pub fn selection_sort<T: Ord + Copy>(list: Vec<T>) -> Vec<T> {
    // Make a copy of the list.
    // Copy will be mutated.
    let mut sorted: Vec<T> = list;

    // Swap for n -1 items in the list
    // If Na and Nb are same, their positions won't change.
    for i in 0..sorted.len() - 1 {
        let smallest = smallest(&sorted[i..]);
        sorted.swap(i, i + smallest);
    }

    // Return the sorted list.
    sorted
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_selection_sort() {
        assert_eq!(smallest(&vec![5, 3, 6, 2, 10]), 3);
        assert_eq!(selection_sort(vec![5, 3, 6, 2, 10]), vec![2, 3, 5, 6, 10]);
        assert_eq!(
            selection_sort(vec![50, 3, 33, 87, 6, 2, 100]),
            vec![2, 3, 6, 33, 50, 87, 100]
        );
    }
}
