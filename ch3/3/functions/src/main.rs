fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value}{unit_label}");
}

fn main() {
    print_labeled_measurement(5, 'h');

    let x = five();
    println!("The value of x is {x}");

    let y = plus_one(x);
    println!("The value of y is {y}");
}

// order of functions doesn't matter
// function parameters MUST have a type annotation
fn another_function(x: i32) {
    println!("The value of x is {x}");
}

// functions can return values but their type must be specified with a -> type annotation
// the value returned is the last expression without a semicolon
// you can return early with the 'return' keyword
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // if instead we had:
    // x + 1;
    // this would cause an error because the function is expected to return an i32
    // but statements don't return a value
}
