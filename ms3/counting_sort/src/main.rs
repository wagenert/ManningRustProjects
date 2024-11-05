mod prng;

use core::fmt;
use std::io::{self, Write};

use prng::Prng;

#[derive(Default, Clone)]
struct Customer {
    id: String,
    num_purchases: i32,
}

impl PartialEq for Customer {
    fn eq(&self, other: &Self) -> bool {
        self.num_purchases == other.num_purchases
    }
}

impl PartialOrd for Customer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.num_purchases.partial_cmp(&other.num_purchases)
    }
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.num_purchases)
    }
}

fn make_random_vec(num_items: i32, max: i32) -> Vec<Customer> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<Customer> = Vec::with_capacity(num_items as usize);

    for i in 0..num_items {
        vec.push(Customer {
            id: format!("C{}", i),
            num_purchases: prng.next_i32(0, max),
        });
    }
    vec
}

// Print at most num_items items.
fn print_vec(vec: &[Customer], num_items: i32) {
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
        .for_each(|x| string.push_str(&format!(" {}", x)));
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

fn counting_sort(v: &[Customer], max: i32) -> Vec<Customer> {
    let mut counts = vec![0; (max + 1) as usize];
    let mut sorted = vec![Customer::default(); v.len()];

    // Count the number of occurrences of each value.
    v.iter().for_each(|x| counts[x.num_purchases as usize] += 1);

    // Modify counts to contain the number of elements <= i.
    for i in 1..counts.len() {
        counts[i] += counts[i - 1];
    }

    // Build the sorted array.
    for i in (0..v.len()).rev() {
        let index = counts[v[i].num_purchases as usize] - 1;
        sorted[index] = v[i].clone();
        counts[v[i].num_purchases as usize] -= 1;
    }
    sorted
}

fn check_sorted(vec: &[Customer]) {
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
    let vec = make_random_vec(num_items, max);

    let sorted_vec = counting_sort(&vec, max);

    check_sorted(&sorted_vec);

    print_vec(&sorted_vec, 10);
}
