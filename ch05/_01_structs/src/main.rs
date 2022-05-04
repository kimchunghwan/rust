use std::fmt;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User.username: {}", self.username)
    }
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user: {}", user1.to_string());
}
