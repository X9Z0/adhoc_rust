use std::io;

fn fib_recursive(n: i32) -> i32 {
    if n < 2 {
        n
    } else {
        fib_recursive(n - 1) + fib_recursive(n - 2)
    }
}
enum Choose {
    One = 1,
    Two = 2,
}

fn choice(n: u8) -> Option<Choose> {
    match n {
        1 => Some(Choose::One),
        2 => Some(Choose::Two),
        _ => None,
    }
}

pub fn main() {
    loop {
        println!("Choose what you want to do");
        println!("1. find fibonacci of the nth number (0 indexing)");
        println!("2. exit the program");
        println!("choose between 1 and 2");
        let mut choose = String::new();
        io::stdin()
            .read_line(&mut choose)
            .expect("Failed to read the input");
        let choose: u8 = match choose.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };
        match choice(choose) {
            Some(Choose::One) => {
                println!("Enter the nth number to find");
                let mut number_input = String::new();
                io::stdin()
                    .read_line(&mut number_input)
                    .expect("Failed to take the input");
                let num: i32 = match number_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Input of the number was invalid");
                        continue;
                    }
                };
                println!("Answer {}", fib_recursive(num));
            }
            Some(Choose::Two) => {
                println!("exiting the program");
                break;
            }
            None => {
                println!("invalid choice");
                continue;
            }
        }
    }
}
