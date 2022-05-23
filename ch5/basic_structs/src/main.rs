struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "User {{ active: {}, username: {}, email: {}, sign_in_count: {} }}", self.active, self.username, self.email, self.sign_in_count)
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user1);

    println!("{}", user1.email);

    //user1.email = String::from("anotheremail@example.com"); // can't do this because the struct was not declared as mutable

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{}", user2.email);

    user2.email = String::from("anotheremail@example.com");
    println!("{}", user2.email);

    let user3 = build_user(String::from("myemail@email.com"), String::from("myusername"));
    println!("{}", user3);

    let user4 = User {
        email: String::from("mynewemail@email.com"),
        ..user3
    };
    println!("{}", user4);


    let black = Color(0, 0, 0);
    println!("{} {} {}", black.0, black.1, black.2);

    let origin = Point(0, 0, 0);
    println!("{} {} {}", origin.0, origin.1, origin.2);


    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}