fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("s1 = {}, s2 = {}", s1, s2);

    let a1 = String::from("hello");
    let a2 = a1.clone();

    println!("a1 = {}, a2 = {}", a1, a2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s);

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
