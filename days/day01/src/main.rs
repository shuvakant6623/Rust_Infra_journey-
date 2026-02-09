// Move vs Copy 
fn main () {
    let a = String::from("infra");
    let b = a;

    println!("{}",b);

    //println!("{}",a); 
    // this will throw error 
}

// Clone Example 

fn clone_example(){
    let a = String::from("rust");
    let b = a.clone();

    println!("{), {}", a, b);
}

// Ownership in Functions 
fn take(s: String) {
    println!("{}", s);
}

fn give()-> String {
    String::from("data")
}

fn ownership_fn(){
    let s = give();
    take(s);
}

// Borrowing

fn borrow(s: String) {
    println!("Length: {}", s.ken());
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