# Sorting Library

This library provides a collection of sorting functions implemented in Rust. It includes quick sort, insertion sort, selection sort, and merge sort algorithms.

## Usage

To use this library, add it to your `Cargo.toml` file as a dependency:

![alt text](https://github.com/SoftDev-Abay/blockchain2_lab1/blob/main/imgs/consumer_out.png?raw=true)

```toml
[dependencies]
sorting_library = "0.1.0"
```

Then import the sorting functions in your Rust file:

```
use sorting_library::{quick_sort, insertion_sort, selection_sort, merge_sort};
```

Here you can see a sorting_consumer output image

Code runs a loop folowing:

1. Accepts a list of numbers separated by space.
2. Asks which sorting to use.
3. Shows sorted array.
4. Prints time taken

## Examples

You can see full example of usage in sorting_consumer/src/main.rs

```
use sorting_library::quick_sort;

fn main() {
    let mut numbers = vec![10, 5, 3, 8, 2, 6, 4, 1, 9, 7];
    quick_sort(&mut numbers);
    println!("Sorted array: {:?}", numbers);
}
```

Replace quick_sort with insertion_sort, selection_sort, or merge_sort to use other sorting algorithms provided by the library.

## Tests

You can run tests for sorting library using

```
cd sorting_library
cargo build
cargo test
```

![alt text](https://github.com/SoftDev-Abay/blockchain2_lab1/blob/main/imgs/tests.png?raw=true)
