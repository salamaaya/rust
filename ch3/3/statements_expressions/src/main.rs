// Statements are instructions that perform some action and do not return a value.
// Expressions evaluate to a resultant value.

fn main() {
    // function definitions are also statements
    let y = 6; // 'let' keyword to assign values to a variable is a statement
    // '6' is an expression
    // calling a function, new scope block, or macro is an expression
    let z = {
        let x = 3;
        x + 1 // expressions do not include semicolons
        // adding semicolon here would make it a statement and not an expression
    };
    println!("The value of z is {z}");

    // statements do NOT return values
    //  let x = (let y = 6); -> errorrrr
}
