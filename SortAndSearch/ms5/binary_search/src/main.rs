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

fn binary_search(vec: &[i32], target: i32) -> (i32, i32) {
    if target < vec[0] || target > vec[vec.len() - 1] {
        return (-1, 0);
    }

    let mut high = vec.len() - 1;
    let mut low = 0;
    let mut test_count = 0;
    loop {
        test_count += 1;
        let i = low + (high - low) / 2;

        if vec[i] == target {
            return (i as i32, test_count);
        }

        if vec[i] < target {
            low = i;
        } else {
            high = i;
        }

        if (high - low) <= 1 {
            if vec[low] == target {
                return (low as i32, test_count + 1);
            }
            if vec[high] == target {
                return (high as i32, test_count + 1);
            }
            return (-1, test_count);
        }
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
    let num_items = get_i32("Items: ");
    let max = get_i32("Max: ");
    let mut vec: Vec<i32> = make_random_vec(num_items, max);
    quick_sort(&mut vec);
    print_vec(&vec, vec.len());
    loop {
        let item_to_search = get_i32("Target (-1 to quit): ");
        if item_to_search == -1 {
            break;
        }

        let (pos, cmp) = binary_search(&vec, item_to_search);
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
        let (pos, cmps) = binary_search(&vec, item_to_search);
        assert_eq!(pos, 4);
        assert_eq!(cmps, pos + 1);
    }

    #[test]
    fn test_linear_search_big_vector() {
        let vec = make_random_vec(1000, 100);
        let item_to_search = 5;
        let (pos, _) = binary_search(&vec, item_to_search);
        assert!(vec[pos as usize] == item_to_search);
    }
}
