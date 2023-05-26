use std::io;

fn main() {
    // for int in 0..15 {
    //     println!("fibo ({}) => {}", int, fib(int))
    // }
    'fibnum loop {
        println!("enter number");

        let mut fibnum = String::new();

        io::stdin()
            .read_line(&mut fibnum)
            .expect("Failed to read line");

        let fibnum: u64 = match fibnum.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if fibnum <= 0 {
            println!("bad number bad bad");
            break 'fibnum;
        }

        println!("Fibonacci of {} is {}", fibnum, fib(fibnum));
        break;
    }
}

fn fib(n: u64) -> u64 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
