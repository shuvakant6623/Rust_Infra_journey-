// This function will NOT compile.
// It returns a reference to a local variable.
// Uncomment to see error.

/*
fn bad_reference() -> &String {
    let s = String::from("infra");
    &s
}
*/

// Fixed version
fn good_reference() -> String {
    let s = String::from("infra");
    s
}

// Longest string with lifetime

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Struct with lifetime

struct Config<'a> {
    name: &'a str,
}

// Impl with lifetime

impl<'a> Config<'a> {
    fn get_name(&self) -> &'a str {
        self.name
    }
}

// Nested scope example

fn nested_scope() {
    let outer = String::from("rust-infra");

    let result: &str;

    {
        let inner = String::from("systems");
        // result = &inner;  // ❌ This will fail
        result = &outer;    // ✅ Safe
    }

    println!("Result: {}", result);
}

// Multiple borrows example

fn borrow_examples() {
    let s = String::from("feature-store");

    // Immutable borrows
    let r1 = &s;
    let r2 = &s;

    println!("{} {}", r1, r2);

    // Mutable borrow (must be alone)
    let mut s2 = String::from("engine");

    let r3 = &mut s2;
    r3.push_str("-core");

    println!("{}", r3);
}

// ---------- Main ----------

fn main() {
    println!("--- Day 2: Lifetimes & Borrowing ---");

    // Exercise 1
    let s = good_reference();
    println!("Good reference: {}", s);

    // Exercise 2
    let a = "distributed";
    let b = "systems";
    let res = longest(a, b);

    println!("Longest: {}", res);

    // Exercise 3 + 4
    let name = String::from("infra-engine");

    let cfg = Config {
        name: &name,
    };

    println!("Config name: {}", cfg.get_name());

    // Exercise 5
    nested_scope();

    // Exercise 6
    borrow_examples();

    println!("--- End of Day 2 ---");
}
