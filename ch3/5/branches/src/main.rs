fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // the following causes an error because
    // number is not a bool
    // if number {
    //      println!("number was three");
    // }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // from the book:
    // Using too many else if expressions can clutter your code,
    // so if you have more than one, you might want to refactor your code.

    let number = if number == 3 { 5 } else { 6 };

    // the following causes an error becuase of type mismatch
    // let number = if number == 3 { 5 } else { "six" };
}
