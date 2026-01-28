fn read(y: bool) {
    if y {
        println!("y is true;");
    }
}

fn plus_one(x: i32) -> i32 {
    x + 1 // x is on the stack (in the frame of plus_one)
}

fn main() {
    // this is safe
    let x = true;
    read(x);

    let n = 5; // n goes on the stack (in the frame of main)
    let y = plus_one(n); // y goes on the stack (in the frame of main)
    print!("the value of y is {y}");

    let a = 5; // a on the stack (in the frame of main)
    // value of a is copied to b (in the frame of main)
    let mut b = a; // b on the stack (in the frame of main)
    b += 1; // b's value is modified (in the frame of main)
    println!("the value of b is {b}");

    let a = [0; 1_000_000]; // 1,000,000 0's on the stack (in the frame of main)
    let b = a; // a copy of 1,000,000 0's on the stack (in the frame of main)

    // Heap
    let a = Box::new([0; 1_000_000]); // 1,000,000 0's on the heap (in the frame of main)
    // a is on the stack (in the frame of main) which points to the heap
    let b = a; // copies pointer from a onto b, a no longer points to the heap,
    // instead b points to the heap

    // taken from the book:
    // When a is bound to Box::new([0; 1_000_000]), we say that a owns the box.
    // The statement let b = a moves ownership of the box from a to b
    // Box deallocation principle: If a variable owns a box,
    // when Rust deallocates the variable’s frame, then Rust deallocates the box’s heap memory.

    // Rust Does Not Permit Manual Memory Management
    // the below does NOT compile
    // let b = Box::new([0; 100]);
    // free(b);
    // assert!(b[0] == 0); // undefined behavior, pointer used after free

    let a_num = 4; // a_num is on the stack (in the frame of main)
    make_and_drop(); // there's no a_box in the heap at this point

    let first = String::from("Ferris"); // points to heap
    let full = add_suffix(first); // at this point, first is deallocated
    println!("{full}");

    // this will NOT compile
    // let first = String::from("Ferris");
    // let full = add_suffix(first);
    // println!("{full}, originally {first}"); // trying to use first after it's been moved and deallocated

    // deep copy
    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}

fn add_suffix(mut name: String) -> String {
    // first no longer owns the box, it gets deallocated
    name.push_str(" Jr.");
    name // return heap pointer to caller
}

fn make_and_drop() {
    let a_box = Box::new(5); // a_box is on the stack (in the frame of make_and_drop)
    // a_box points to the heap
    // when a_box goes out of scope, the heap is deallocated
}
