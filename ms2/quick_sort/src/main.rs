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
    return vec;
}

// Print at most num_items items.
fn print_vec(vec: &Vec<i32>, num_items: i32) {
    let mut max = vec.len();
    if max > num_items as usize {
        max = num_items as usize;
    }

    let mut string = String::new();
    string.push_str("[");

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    for i in 1usize..max {
        string.push_str(" ");
        string.push_str(&vec[i].to_string());
    }
    string.push_str("]");
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
    return trimmed.parse::<i32>()
        .expect("Error parsing integer");
}

fn partition(vec:&mut [i32]) -> usize {
    let hi = vec.len() - 1;
    let pivot = vec[hi];
    let mut i = hi;
    for j in 0..(hi - 1) {
        while vec[j] > pivot  {
            i -= 1;
            (vec[i], vec[j]) = (vec[j], vec[i]);
        }
    }
    i-=1;
    (vec[i], vec[hi]) = (vec[hi], vec[i]);
    return i;
}

fn quick_sort(vec:&mut [i32]) {
    match vec.len() {
        0 => 
            return,
        1 => 
            return,
        2 => {
            if vec[0] > vec[1] {
                (vec[0], vec[1]) = (vec[1], vec[0]);
            } 
            return;
        },
        _ => {
            let p = partition(vec);
            quick_sort(&mut vec[0..p]);
            quick_sort(&mut vec[(p + 1)..]);
        }
    }

}

fn main() {
    let num_items = get_i32("Please specify number of items to be sorted: ");
    let max = get_i32("Please specify the maximum value for an item: ");
    let mut vec = make_random_vec(num_items, max);
    print_vec(&vec, num_items);
    quick_sort(vec.as_mut_slice());
    print_vec(&vec, num_items);
    //check_sorted(&vec); 
}
