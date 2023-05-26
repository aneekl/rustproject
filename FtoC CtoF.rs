use std::io;
// use std::cmp::Ordering;

fn main() {
    println!("F and C converter");

    loop {
        println!("1. F to C");
        println!("2. C to F");

        let mut degree = String::new();

        io::stdin()
            .read_line(&mut degree)
            .expect("Fail to read line");

        let degree: u32 = match degree.trim() {
            "1" => 1,
            "2" => 2,
            _ => {
                println!("only 1 or 2! \n");
                continue;
            }
        };

        if degree == 1 {
            println!("now enter fahrenheit value");
        } else {
            println!("now enter celsius value");
        }

        let mut value = String::new();

        io::stdin()
            .read_line(&mut value)
            .expect("Fail to read line");

        let value: f32 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("enter valid value!!");
                continue;
            }
        };

        // println!("Your value is {value}");

        let converted = if value == 1. {
            (value - 32.) * (5. / 9.)
        } else {
            (value * (9. / 5.)) + 32.
        };

        if degree == 1 {
            println!("{converted} degree celsius");
            break;
        } else {
            println!("{converted} degree fahrenheit");
            break;
        }
    }
}
