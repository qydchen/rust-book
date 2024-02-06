enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    let x: i8 = 5;
    let y: Option<i8> = Some(10);

    match y {
        Some(y) => println!("{}", y + x),
        None => println!("none"),
    }
}
