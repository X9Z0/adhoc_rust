use std::io;

fn ctof(x: f64) -> f64 {
    (9 / 5.0 * x) + 32
}

fn ftoc(x: f64) -> f64 {
    5 / 9.0 * (x - 32)
}

enum Choose {
    One = 1,
    Two = 2,
    Three = 3,
}

fn choice(value: u8) -> Option<Choose> {
    match value {
        1 => Some(Choose::One),
        2 => Some(Choose::Two),
        3 => Some(Choose::Three),
        _ => None,
    }
}

fn main() {
    let mut choose: u8;
    loop {
        println!("Please choose what you want to convert");
        println!("1. C -> F");
        println!("2. F -> C");
        println!("3. Exit");
        println!("Choose a number from (1 to 3)");

        io::stdin()
            .read_line(&mut choose)
            .expect("Failed to read the input");

        let mut number: f64;

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read the input");

        let answer: f64 = match choice(choose) {
            Some(Choose::One) => ctof(num),
            Some(Choose::Two) => ftoc(num),
            Some(Choose::Three) => {
                println!("exiting the program");
                break;
            }
            None => {
                println!("Invalid choice");
                continue;
            }
        };
    }
}
