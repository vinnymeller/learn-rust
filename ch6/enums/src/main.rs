enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl std::fmt::Display for IpAddrKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            IpAddrKind::V4 => write!(f, "IPv4"),
            IpAddrKind::V6 => write!(f, "IPv6"),
        }
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn addr_type(addr: IpAddr2) -> String {
    match addr {
        IpAddr2::V4(..) => "IPv4".to_string(),
        IpAddr2::V6(..) => "IPv6".to_string(),
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{} {}", loopback.kind, loopback.address);

    let home_addr = IpAddr2::V4(127, 0, 0, 1);
    let loopback_addr = IpAddr2::V6(String::from("::1"));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("IPv4"),
        IpAddrKind::V6 => println!("IPv6"),
    }
}
