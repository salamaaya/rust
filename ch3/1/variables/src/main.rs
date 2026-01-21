fn main() {
    let x = 5;

    let x = x + 1; // shadowing
    
    { // inner scope
        let x = x * 2; // inner shadowing
        println!("The value of x is {x}");
    } // end of inner shadowing, x returns to its previous value

    println!("The value of x is {x}");
}
