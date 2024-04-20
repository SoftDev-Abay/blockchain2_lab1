fn main() {
    let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    sorting_library::insertion_sort(&mut numbers);
    println!("Sorted numbers: {:?}", numbers);
}
