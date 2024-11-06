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

    vec[0..max]
        .iter()
        .for_each(|&x| string.push_str(format!(" {}", &x).as_str()));
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

fn bubble_sort(vec: &mut [i32]) {
    for i in 1..vec.len() {
        for j in 0..(vec.len() - i) {
            if vec[j] > vec[j + 1] {
                vec.swap(j + 1, j);
            }
        }
    }
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

fn main() {
    let num_items = get_i32("Please specify number of items to be sorted: ");
    let max = get_i32("Please specify the maximum value for an item: ");
    let mut vec = make_random_vec(num_items, max);
    print_vec(&vec, 10);
    bubble_sort(&mut vec);
    print_vec(&vec, 10);
    check_sorted(&vec);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_random_vec() {
        let num_items = 10;
        let max = 100;
        let vec = make_random_vec(num_items, max);
        assert_eq!(vec.len(), num_items as usize);
        for &x in vec.iter() {
            assert!(x >= 0 && x <= max);
        }
    }

    #[test]
    fn test_bubble_sort() {
        let mut vec = vec![3, 2, 1];
        bubble_sort(&mut vec);
        assert_eq!(vec, vec![1, 2, 3]);
    }

    #[test]
    fn test_check_sorted() {
        let vec = vec![1, 2, 3];
        check_sorted(&vec);
        let vec = vec![3, 2, 1];
        check_sorted(&vec);
    }

    #[test]
    fn test_check_identical() {
        let num_items = 10;
        let max = 100;
        let vec = make_random_vec(num_items, max);
        let mut vec2 = vec.clone();
        bubble_sort(&mut vec2);
        let (is_identical, residual) =
            vec.iter()
                .fold((true, vec2), |(is_identical, mut residual), &x| {
                    if is_identical {
                        if let Some(pos) = residual.iter().position(|&y| y == x) {
                            residual.remove(pos);
                            (is_identical, residual)
                        } else {
                            (false, residual)
                        }
                    } else {
                        (is_identical, residual)
                    }
                });
        assert!(is_identical && residual.is_empty());
    }
}
