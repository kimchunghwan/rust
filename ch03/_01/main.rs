fn main() {
    let x = 5;

    let x = x + 1;
    let spaces = "   ";
    let spaces = spaces.len();
    println!("len, {}", spaces);
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
