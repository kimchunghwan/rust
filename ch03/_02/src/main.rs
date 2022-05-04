use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("please enter an array index.");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("failed read line");
    let index: usize = index.trim().parse().expect("index entered was not number");
    println!("index {} array value {}", index, a[index])
}
