use std::io::{self, Write};

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    print!("Введіть число: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Не вдалося прочитати рядок");

    let num: u32 = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Будь ласка, введіть дійсне число.");
            return;
        }
    };

    if is_prime(num) {
        println!("{} є простим числом.", num);
    } else {
        println!("{} не є простим числом.", num);
    }
}
