mod prng;

use std::io::{self, Write};

use prng::Prng;

fn make_random_vec(num_items: i32, max: i32) -> Vec<i32> {
    // Prepare a Prng.
    let mut prng = Prng::new();

    let mut vec: Vec<i32> = Vec::with_capacity(num_items as usize);

    for _i in 0..num_items {
        vec.push(prng.next_i32(0, max));
    }
    vec
}

// Print at most num_items items.
fn print_vec(vec: &[i32], num_items: usize) {
    let mut max = vec.len();
    if max > num_items {
        max = num_items;
    }
    let mut string = String::new();
    string.push('[');

    if max > 0usize {
        string.push_str(&vec[0].to_string());
    }

    vec[0..max]
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

fn linear_search(vec: &[i32], item_to_search: i32) -> Vec<i32> {
    let mut result = Vec::new();

    for i in 0..vec.len() {
        if vec[i] == item_to_search {
            result.push(i as i32);
        }
    }
    result
}

fn main() {
    let num_items = get_i32("Please specify number of items to be sorted: ");
    let max = get_i32("Please specify the maximum value for an item: ");
    let item_to_search = get_i32("Please specify the item to search for: ");
    let vec: Vec<i32> = make_random_vec(num_items, max);

    let sorted_vec: Vec<i32> = linear_search(&vec, item_to_search);

    println!("Positions of the item to search for: ");
    print_vec(&sorted_vec, sorted_vec.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let item_to_search = 5;
        let result = linear_search(&vec, item_to_search);
        assert_eq!(result, vec![4]);
    }

    #[test]
    fn test_linear_search_big_vector() {
        let vec = make_random_vec(1000, 100);
        let item_to_search = 5;
        let result = linear_search(&vec, item_to_search);
        assert!(result.iter().all(|x| vec[(*x) as usize] == item_to_search));
    }
}
