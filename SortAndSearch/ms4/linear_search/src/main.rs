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

fn linear_search(vec: &[i32], target: i32) -> (i32, i32) {
    if let Some(pos) = vec.iter().position(|&x| x == target) {
        return (pos as i32, (pos + 1) as i32);
    }
    (-1, vec.len() as i32)
}

fn main() {
    let num_items = get_i32("Items: ");
    let max = get_i32("Max: ");
    let vec: Vec<i32> = make_random_vec(num_items, max);
    print_vec(&vec, vec.len());
    loop {
        let item_to_search = get_i32("Target (-1 to quit): ");
        if item_to_search == -1 {
            break;
        }

        let (pos, cmp) = linear_search(&vec, item_to_search);
        if pos == -1 {
            println!("Target {} not found, {} tests", item_to_search, cmp);
        } else {
            println!("numbers[{}] = {}, {} tests", pos, item_to_search, cmp);
        }
        println!("Positions of the item to search for: ");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let item_to_search = 5;
        let (pos, cmps) = linear_search(&vec, item_to_search);
        assert_eq!(pos, 4);
        assert_eq!(cmps, pos + 1);
    }

    #[test]
    fn test_linear_search_big_vector() {
        let vec = make_random_vec(1000, 100);
        let item_to_search = 5;
        let (pos, _) = linear_search(&vec, item_to_search);
        assert!(vec[pos as usize] == item_to_search);
    }
}
