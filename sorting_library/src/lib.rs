
pub fn quick_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = arr[arr.len() / 2];
    let (mut i, mut j) = (0, arr.len() - 1);
    while i <= j {
        while arr[i] < pivot {
            i += 1;
        }
        while arr[j] > pivot {
            j -= 1;
        }
        if i <= j {
            arr.swap(i, j);
            i += 1;
            if j > 0 {
                j -= 1;
            }
        }
    }
    if j > 0 {
        quick_sort(&mut arr[0..=j]);
    }
    if i < arr.len() {
        quick_sort(&mut arr[i..]);
    }
}

pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_idx = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
    }
}

pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    let mid = arr.len() / 2;
    if mid == 0 {
        return;
    }
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut ret = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut ret[..]);
    arr.copy_from_slice(&ret);
}

fn merge<T: Ord + Copy>(left: &[T], right: &[T], ret: &mut [T]) {
    let (mut left_idx, mut right_idx, mut ret_idx) = (0, 0, 0);

    while left_idx < left.len() && right_idx < right.len() {
        if left[left_idx] <= right[right_idx] {
            ret[ret_idx] = left[left_idx];
            left_idx += 1;
        } else {
            ret[ret_idx] = right[right_idx];
            right_idx += 1;
        }
        ret_idx += 1;
    }

    if left_idx < left.len() {
        ret[ret_idx..].copy_from_slice(&left[left_idx..]);
    }
    if right_idx < right.len() {
        ret[ret_idx..].copy_from_slice(&right[right_idx..]);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::time::Instant;




    fn generate_random_array(size: usize) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        (0..size).map(|_| rng.gen_range(0..100)).collect()
    }

    fn sort_and_measure_time<F>(mut arr: Vec<i32>, sort_fn: F) -> (Vec<i32>, u128)
    where
        F: Fn(&mut [i32]),
    {
        let start = Instant::now();
        sort_fn(&mut arr);
        let duration = start.elapsed().as_micros();
        (arr, duration)
    }

    fn run_sort_test<F>(sort_name: &str, sort_fn: F)
    where
        F: Fn(&mut [i32]),
    {
        let arr = generate_random_array(10);
        println!("\nRandom Array for {}: {:?}", sort_name, arr);

        let (sorted_arr, duration) = sort_and_measure_time(arr, sort_fn);
        println!("Sorted Array for {}: {:?}", sort_name, sorted_arr);
        println!("Time taken for {} sort: {} microseconds\n", sort_name, duration);

        for i in 0..sorted_arr.len() - 1 {
            assert!(sorted_arr[i] <= sorted_arr[i + 1]);
        }
    }

    #[test]
    fn test_sorting_algorithms() {
        run_sort_test("Quick Sort", quick_sort);
        run_sort_test("Insertion Sort", insertion_sort);
        run_sort_test("Selection Sort", selection_sort);
        run_sort_test("Merge Sort", merge_sort);
    }
}
