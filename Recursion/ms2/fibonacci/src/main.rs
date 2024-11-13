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

fn fibonacci(n: i64) -> i64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
fn main() {
    println!("Enter -1 to exit\n");
    loop {
        let n = get_i64("N:");

        if n < 0 {
            break;
        }

        println!("fibonacci({}) = {}", n, fibonacci(n));
    }
}
