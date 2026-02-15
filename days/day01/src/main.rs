// Move vs Copy
fn main() {
    move_vs_copy();
    clone_example();
    ownership_fn();
    borrowing_example();
    mut_borrow();
}

// Move example
fn move_vs_copy() {
    let a = String::from("infra");
    let b = a; // ownership moved to b

    println!("{}", b);

    // println!("{}", a); //  would error (moved)
}

// Clone Example
fn clone_example() {
    let a = String::from("rust");
    let b = a.clone(); // deep copy

    println!("{}, {}", a, b); // both valid
}

// Ownership in Functions
fn take(s: String) {
    println!("Taken: {}", s);
} // s dropped here

fn give() -> String {
    String::from("data")
}

fn ownership_fn() {
    let s = give();   // ownership received
    take(s);          // ownership moved
    // println!("{}", s); //  would error
}


// Borrowing (Immutable)

fn borrow(s: &String) {
    println!("Length: {}", s.len());
}

fn borrowing_example() {
    let s = String::from("system");

    borrow(&s);       
    println!("{}", s); 
}

// Mutable Borrow
fn mut_borrow() {
    let mut s = String::from("ml");

    let r = &mut s;
    r.push_str(" infra");

    println!("{}", s);
}

