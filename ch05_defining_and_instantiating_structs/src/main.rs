use std::fmt;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) ->fmt::Result {
        write!(f,"test {} {} {} {}", self.active, self.username, self.email, self.sign_in_count)
    }

}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) ->u32 {
        self.width*self.height
    }
}

fn main() {
    let user = User {
        email: String::from("test@test.com"),
        username: String::from("test"),
        active: true,
        sign_in_count: 1
    };
    println!("user: {}", user.to_string());

    let rect = Rectangle{
        width: 4,
        height: 4
    };
    println!("area: {}", rect.area())
}
