use std::io::{self, Write};

fn get_i64(prompt: &str) -> i64 {
    print!("{prompt} ");
    io::stdout().flush().unwrap();

    let mut str_value = String::new();
    io::stdin()
        .read_line(&mut str_value)
        .expect("Error reading input");

    let trimmed = str_value.trim();
    trimmed.parse().expect("Error parsing integer")
}

fn fibonacci_on_the_fly(values: &mut Vec<i64>, n: i64) -> i64 {
    if values.len() > n as usize {
        return values[n as usize];
    }
    let fibonacci_of_n = fibonacci_on_the_fly(values, n - 1) + fibonacci_on_the_fly(values, n - 2);
    values.push(fibonacci_of_n);
    values[n as usize]
}

fn prefill_vector() -> Vec<i64> {
    let mut values: Vec<i64> = vec![0, 1];
    for i in 2..93 {
        values.push(values[i - 1] + values[i - 2]);
    }
    values
}

fn fibonacci_bottom_up(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}

fn main() {
    // Initialize the prefilled vector.
    let prefilled_values = prefill_vector();

    // Create a vector for fill-on-the-fly.
    let mut fill_on_the_fly_values: Vec<i64> = vec![0, 1];

    loop {
        // Prompt the user for n.
        let n = get_i64("N: ");
        if n < 0 {
            break;
        }

        // Calculate the Fibonacci number.
        println!("Prefilled:  {}", prefilled_values[n as usize]);
        println!(
            "On the fly: {}",
            fibonacci_on_the_fly(&mut fill_on_the_fly_values, n)
        );
        println!("Bottom up:  {}", fibonacci_bottom_up(n));
        println!();
    }
}
