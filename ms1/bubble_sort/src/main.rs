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

    vec.iter().for_each(|&x| string.push_str(&x.to_string()));
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
    print_vec(&vec, num_items);
    bubble_sort(&mut vec);
    print_vec(&vec, num_items);
    check_sorted(&vec);
}
