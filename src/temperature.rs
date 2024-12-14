use std::io;

fn ctof(x: f64) -> f64 {
    (9.0 / 5.0 * x) + 32.0
}

fn ftoc(x: f64) -> f64 {
    5.0 / 9.0 * (x - 32.0)
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

pub fn main() {
    loop {
        println!("Please choose what you want to convert:");
        println!("1. C -> F");
        println!("2. F -> C");
        println!("3. Exit");
        println!("Choose a number from (1 to 3):");

        let mut choose_input = String::new();
        io::stdin()
            .read_line(&mut choose_input)
            .expect("Failed to read input");

        let choose: u8 = match choose_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number between 1 and 3.");
                continue;
            }
        };

        match choice(choose) {
            Some(Choose::One) => {
                println!("Enter the temperature in Celsius:");
                let mut temp_input = String::new();
                io::stdin()
                    .read_line(&mut temp_input)
                    .expect("Failed to read input");
                let temp: f64 = match temp_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid temperature, please try again.");
                        continue;
                    }
                };
                println!("The converted temperature is: {:.2} °F", ctof(temp));
            }
            Some(Choose::Two) => {
                println!("Enter the temperature in Fahrenheit:");
                let mut temp_input = String::new();
                io::stdin()
                    .read_line(&mut temp_input)
                    .expect("Failed to read input");
                let temp: f64 = match temp_input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid temperature, please try again.");
                        continue;
                    }
                };
                println!("The converted temperature is: {:.2} °C", ftoc(temp));
            }
            Some(Choose::Three) => {
                println!("Exiting the program.");
                break;
            }
            None => {
                println!("Invalid choice, please enter a number between 1 and 3.");
            }
        }
    }
}
