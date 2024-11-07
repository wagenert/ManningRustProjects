pub(crate) mod prng;

//use core::num;
use std::io;
use std::io::Write;

use prng::Prng;

fn make_random_vec(num_items: i32, max: i32) -> Vec<i32> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);
    for _ in 0..num_items {
        vec.push(prng.next_i32(0, max));
    }
    vec
}

// Print at most num_items items.
fn print_vec(vec: &[i32], num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push('[');

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    vec[0..num_items as usize]
        .iter()
        .for_each(|&x| string.push_str(&format!(" {}", x)));
    string.push(']');
    println!("{string}");
}
// ...
// Prompt the user for an i32.
fn get_i32(prompt: &str) -> i32 {
    print!("{prompt}");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    trimmed.parse::<i32>().expect("Error parsing integer")
}

fn check_sorted(vec: &[i32]) {
    let mut sorted = true;
    let mut i = 0;
    while sorted && i < vec.len() - 1 {
        sorted = vec[i] <= vec[i + 1];
        i += 1;
    }
    if sorted {
        println!("The vector is sorted!");
    } else {
        println!("The vector is NOT sorted!");
    }
}

fn partition(vec: &mut [i32]) -> usize {
    let hi = vec.len() - 1;
    let pivot = vec[hi];
    let (mut lower, mut higher) =
        vec[0..hi]
            .iter()
            .fold((Vec::new(), Vec::new()), |(mut lower, mut bigger), &x| {
                if x < pivot {
                    lower.push(x);
                } else {
                    bigger.push(x);
                }
                (lower, bigger)
            });
    let pos = lower.len();
    lower.push(pivot);
    lower.append(&mut higher);
    vec.copy_from_slice(&lower);

    pos
}

fn quick_sort(vec: &mut [i32]) {
    match vec.len() {
        0 => (),
        1 => (),
        2 => {
            // if vec is not sorted, swap the two elements.
            if vec[0] > vec[1] {
                vec.swap(0, 1);
            }
        }
        _ => {
            let p = partition(vec);
            let upper_partition = if p == vec.len() - 1 { p } else { p + 1 };
            quick_sort(&mut vec[0..p]);
            quick_sort(&mut vec[upper_partition..]);
        }
    }
}

fn main() {
    let num_items = get_i32("Please specify number of items to be sorted: ");
    let max = get_i32("Please specify the maximum value for an item: ");
    let mut vec = make_random_vec(num_items, max);
    print_vec(&vec, 10);
    quick_sort(&mut vec);
    print_vec(&vec, 10);
    check_sorted(&vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let p = partition(&mut vec);
        assert_eq!(p, 6);
    }

    #[test]
    fn test_quick_sort() {
        let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        quick_sort(&mut vec);
        assert_eq!(vec, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test_vectors_are_identical() {
        let mut vec = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        let vec2 = vec.clone();
        quick_sort(&mut vec);
        let (identical, res) = vec.iter().fold((true, vec2), |(identical, mut vec2), x| {
            if let Some(pos) = vec2.iter().position(|&y| y == *x) {
                vec2.remove(pos);
                (identical, vec2)
            } else {
                (false, vec2)
            }
        });
        assert!(identical && res.is_empty());
    }
}
