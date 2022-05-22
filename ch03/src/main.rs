fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    let mut number = 3; 
    while number > 0 {
        println!("number: {}", number);
        number -=1;
    }

    let arr = [1,2,3,4,5];

    for elem in arr {
        println!("elem: {}", elem);
    }
    
    
    for elem in (1..4).rev() {
        println!("rev: {}", elem)
    }
    
    

    println!("process end")
}
