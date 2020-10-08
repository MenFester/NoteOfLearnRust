fn main() {
    let four = IpAddrKind::V4;
    let _ = IpAddrKind::V6;
    println!("{:?}", four);

    let home1 = IpAddr1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("home1 is {:#?}", home1);

    let loopback1 = IpAddr1 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("loopback1 is {:#?}", loopback1);

    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));
    println!("home2 is {:#?}", home2);
    println!("loopback2 is {:#?}", loopback2);

    let home3 = IpAddr3::V4(127, 0, 0, 1);
    let loopback3 = IpAddr3::V6(String::from("::1"));
    println!("home3 is {:#?}", home3);
    println!("loopback3 is {:#?}", loopback3);

    let m = Message::Write(String::from("hello"));
    m.call();
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr1 {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,    // 看做空结构体
    Move { x: i32, y: i32 },    // 看做一般结构体的语法
    Write(String),    // 看做单元素的元组结构体
    ChangeColor(i32, i32, i32),    // 看做元组结构体的语法
}

impl Message {
    fn call(&self) {
        println!("Function call!");
        println!("{:?}", self);
    }
}