use sorting_library;
use std::io;
use std::time::Instant;

fn main() {
    loop {
        println!("Enter a list of numbers separated by spaces (or type 'exit' to quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        if input.trim().eq_ignore_ascii_case("exit") {
            break;
        }
        
        let mut numbers: Vec<i32> = input
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();
        
        if numbers.is_empty() {
            println!("No valid numbers entered, please try again.");
            continue;
        }

        println!("Choose a sorting algorithm:");
        println!("1: Quick Sort");
        println!("2: Insertion Sort");
        println!("3: Selection Sort");
        println!("4: Merge Sort");

        input.clear(); 
        io::stdin().read_line(&mut input).unwrap();
        let choice = input.trim();

        match choice {
            "1" => {
                let start = Instant::now();
                sorting_library::quick_sort(&mut numbers);
                let duration = start.elapsed();
                println!("Sorted with Quick Sort: {:?}", numbers);
                println!("Time taken: {:?}", format_duration(duration));
            }
            "2" => {
                let start = Instant::now();
                sorting_library::insertion_sort(&mut numbers);
                let duration = start.elapsed();
                println!("Sorted with Insertion Sort: {:?}", numbers);
                println!("Time taken: {:?}", format_duration(duration));
            }
            "3" => {
                let start = Instant::now();
                sorting_library::selection_sort(&mut numbers);
                let duration = start.elapsed();
                println!("Sorted with Selection Sort: {:?}", numbers);
                println!("Time taken: {:?}", format_duration(duration));
            }
            "4" => {
                let start = Instant::now();
                sorting_library::merge_sort(&mut numbers);
                let duration = start.elapsed();
                println!("Sorted with Merge Sort: {:?}", numbers);
                println!("Time taken: {:?}", format_duration(duration));
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn format_duration(dur: std::time::Duration) -> String {
    if dur.as_secs() > 0 {
        format!("{} seconds", dur.as_secs())
    } else if dur.as_millis() > 0 {
        format!("{} milliseconds", dur.as_millis())
    } else if dur.as_micros() > 0 {
        format!("{} microseconds", dur.as_micros())
    } else {
        format!("{} nanoseconds", dur.as_nanos())
    }
}
