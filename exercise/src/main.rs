// Given a list of integers, use a vector and return
// the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will
// be helpful here) of the list.

use std::collections::HashMap;

fn calculate_median(numbers: &[i32]) -> Option<i32> {
    let mut sorted = numbers.to_vec();
    sorted.sort();

    let len = sorted.len();
    if len == 0 {
        return None;
    }

    // Odd length: middle element
    if len % 2 == 1 {
        Some(sorted[len / 2])
    }
    // Even length: average of two middle elements
    else {
        Some((sorted[len / 2 - 1] + sorted[len / 2]) / 2)
    }
}

fn calculate_mode(numbers: &[i32]) -> Vec<i32> {
    // Create a HashMap to count occurrences of each number
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();

    // Count how many times each number appears
    for &number in numbers {
        // Increment the count for this number, starting at 1 if not seen before
        *frequency_map.entry(number).or_insert(0) += 1;
    }

    // Find the highest frequency
    let max_frequency = frequency_map.values().max().cloned().unwrap_or(0);

    // Collect all numbers with the highest frequency
    let modes: Vec<i32> = frequency_map
        .iter()
        .filter_map(|(&number, &count)| {
            // If this number's count matches the max frequency, include it
            if count == max_frequency {
                Some(number)
            } else {
                None
            }
        })
        .collect();

    modes
}

// fn median(list: &[i32]) -> i32 {
//     let mut sorted_list = list.clone();
//     sorted_list.sort();
//     let half = (sorted_list.len() / 2) as f32;
//     let index = half.round() as i32;
//     return sorted_list.get(index).unwrap_or(0).clone();
// }

// fn median(list: &[i32]) -> i32 {
//     let mut sorted_list = list.to_vec();
//     sorted_list.sort();
//     let len = sorted_list.len();
//     if len % 2 != 0 {
//         let mid_right = len / 2;
//         let mid_left = mid_right - 1;
//         println!("{mid_left}, {mid_right}");
//         (sorted_list[mid_left] + sorted_list[mid_right]) / 2
//     } else {
//         sorted_list[len / 2]
//     }
// }

// fn mode(list: &[i32]) -> Vec<i32> {
//     let mut occurrences: HashMap<i32, i32> = HashMap::new();
//     for &number in list {
//         *occurrences.entry(number).or_insert(0) += 1;
//     }
//     let max_count = occurrences.values().max().cloned().unwrap_or(0);
//     // println!("{occurrences:?}, {max_count}");
//     let mut result: Vec<i32> = vec![];
//     for (key, value) in &occurrences {
//         if value.clone() == max_count {
//             result.push(key.clone());
//         }
//     }
//     return result;
// }

fn main() {
    let numbers = vec![49, 23, 2, 23, 2, 18, 39];
    let mode_numbers = calculate_mode(&numbers);
    let median = calculate_median(&numbers).unwrap_or(0);
    println!("{median}, {mode_numbers:?}");
}
