// this program generates the nth fibonacci number

use std::io;

fn main() {
    loop {
        println!("Please enter n");

        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line.");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut f1: u64 = 1;
        let mut f2: u64 = 0;
        let mut result: u64 = 0;

        for _ in 1..n {
            result = f1 + f2;
            f2 = f1;
            f1 = result;
        }

        println!("The nth fibonacci number is: {result}");
        break;
    }
}
