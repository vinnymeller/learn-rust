enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

}

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("IPv4"),
        IpAddrKind::V6 => println!("IPv6"),
    }
}
