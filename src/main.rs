// Quick Sort Implementation
pub fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr[arr.len() - 1];
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

// Main function to demonstrate sorting
fn main() {
    let mut arr = [3, 6, 8, 10, 1, 2, 1];
    println!("Original array: {:?}", arr);
    quick_sort(&mut arr);
    println!("Sorted array: {:?}", arr);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_case_sorted_array() {
        // Test sorting of a typical array with integers
        let mut arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn negative_case_invalid_input() {
        // Since Rust's type system enforces type checks, it's not possible to test "incorrect data types" directly.
        // Instead, we can test cases where we provide slices or reference mismatches.
        let mut arr = [1, 2, 3];
        // Testing invalid slice reference, should not panic
        quick_sort(&mut arr[0..0]); // Empty slice should be handled gracefully
        assert_eq!(arr, [1, 2, 3]); // Ensure the array remains unchanged
    }

    #[test]
    fn performance_case_large_array() {
        // Test with a large array for performance
        let mut arr: Vec<i32> = (0..10000).rev().collect(); // Large array, reverse sorted
        quick_sort(&mut arr);
        assert!(is_sorted(&arr)); // The sorted array should be in ascending order
    }

    #[test]
    fn boundary_case_edge_cases() {
        // Empty array
        let mut empty_arr: [i32; 0] = [];
        quick_sort(&mut empty_arr);
        assert_eq!(empty_arr, []);

        // Single-element array
        let mut single_arr = [42];
        quick_sort(&mut single_arr);
        assert_eq!(single_arr, [42]);

        // Array with duplicates
        let mut duplicate_arr = [3, 3, 3, 3, 3];
        quick_sort(&mut duplicate_arr);
        assert_eq!(duplicate_arr, [3, 3, 3, 3, 3]);

        // Already sorted array
        let mut sorted_arr = [1, 2, 3, 4, 5];
        quick_sort(&mut sorted_arr);
        assert_eq!(sorted_arr, [1, 2, 3, 4, 5]);

        // Reverse sorted array
        let mut reverse_arr = [5, 4, 3, 2, 1];
        quick_sort(&mut reverse_arr);
        assert_eq!(reverse_arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn idempotency_case_multiple_runs() {
        // Test that running quick_sort multiple times on the same array produces the same result
        let mut arr = [4, 2, 7, 1, 9, 3];
        quick_sort(&mut arr);
        let sorted_once = arr.clone();
        quick_sort(&mut arr);
        assert_eq!(arr, sorted_once); // Second sort should produce the same sorted array
    }

    // Helper function to check if an array is sorted in ascending order
    fn is_sorted(arr: &[i32]) -> bool {
        for i in 1..arr.len() {
            if arr[i - 1] > arr[i] {
                return false;
            }
        }
        true
    }
}
