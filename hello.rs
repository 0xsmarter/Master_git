use std::io;

fn main() {
    loop {
        println!("Enter first number:");

        let mut first_input = String::new();
        io::stdin()
            .read_line(&mut first_input)
            .expect("Failed to read line");
        let first_num: f64 = match first_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("Enter an operator (+, -, *, /):");
        let mut operator_input = String::new();
        io::stdin()
            .read_line(&mut operator_input)
            .expect("Failed to read line");
        let operator = operator_input.trim();

        println!("Enter second number:");

        let mut second_input = String::new();
        io::stdin()
            .read_line(&mut second_input)
            .expect("Failed to read line");
        let second_num: f64 = match second_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        let result = match operator {
            "+" => first_num + second_num,
            "-" => first_num - second_num,
            "*" => first_num * second_num,
            "/" => {
                if second_num != 0.0 {
                    first_num / second_num
                } else {
                    println!("Cannot divide by zero!");
                    continue;
                }
            },
            _ => {
                println!("Invalid operator!");
                continue;
            },
        };

        println!("The result is: {}", result);
        
        println!("Do you want to perform another calculation? (yes/no):");
        let mut continue_input = String::new();
        io::stdin()
            .read_line(&mut continue_input)
            .expect("Failed to read line");
        if continue_input.trim().to_lowercase() != "yes" {
            break;
        }
    }
}
