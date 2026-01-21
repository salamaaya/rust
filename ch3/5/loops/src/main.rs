fn main() {
    // this goes on forever
    // loop {
    //     println!("again!");
    // }
    // use 'break' to stop the loop

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // the value you want returned after the break expression you use
            // to stop the loop; that value will be returned out of the loop so that you can use it
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // if you have loops within loops, 'break' and 'continue' apply to the inner most loop
    // you label loops with a single quote and break out of the labeled loop
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // while loops
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // print each element of an array
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // for loops
    for element in a {
        println!("the value is: {}", element);
    }
    // can be more efficient than while loops because
    // it doesn't have to check the condition after each iteration
    // even in cases where you want to run a loop a specific number of times
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
