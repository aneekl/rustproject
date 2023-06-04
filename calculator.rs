use std::io;
use std::io::Write;

fn operation(func: u8, x: i32, y: i32) {
    match func {
        1 => println!("{} + {} = {}", x, y, add(x, y)),
        2 => println!("{} - {} = {}", x, y, subtract(x, y)),
        3 => println!("{} * {} = {}", x, y, multiply(x, y)),
        4 => println!("{} / {} = {}", x, y, division(x, y)),
        _ => println!("Error"),
    }
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

fn division(x: i32, y: i32) -> i32 {
    x / y
}

fn validate(app: u8) -> u8 {
    match app {
        1 => 1,
        2 => 2,
        3 => 3,
        4 => 4,
        _ => 0,
    }
}

fn input() -> u8 {
    let mut sel = String::new();
    io::stdin().read_line(&mut sel).expect("fail");
    let app: u8 = sel.trim().parse().expect("not interger");

    app
}

fn inputnum() -> i32 {
    let mut sel = String::new();
    io::stdin().read_line(&mut sel).expect("fail");
    let app: i32 = sel.trim().parse().expect("not interger");

    app
}

fn main() {
    println!("Hello, world!");
    println!("this is calculator");
    println!("select applxation");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. multiply");
    println!("4. division");

    loop {
        print!("select 1/2/3/4: ");
        std::io::stdout().flush().unwrap();
        let app = input();
        // println!("application {}", app);
        let validated = validate(app);

        if validated == 0 {
            println!("not valid number");
            continue;
        }

        print!("enter first: ");
        std::io::stdout().flush().unwrap();
        let x = inputnum();

        print!("enter second: ");
        std::io::stdout().flush().unwrap();
        let y = inputnum();

        operation(validated, x, y);
        break;
    }
    println!("Thank you")
}
