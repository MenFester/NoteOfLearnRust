fn main() {
    let mut s = String::new();
    s.push('H');
    s.push('e');
    s.push('l');
    s.push('l');
    s.push('o');
    println!("{:?}", s);

    let data = "initial contents";
    // data.push('?');    // 报错：method not found in `&str`
    let mut s = data.to_string();
    s.push('!');
    println!("{:?}", s);

    let mut s = "initial contents".to_string();
    s.push('?');
    println!("{:?}", s);
    
    let mut s = String::from("initial content");
    s.push_str(". Are you ok?");
    println!("{:?}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;    // s1已经被移动，再不能被使用。因为+运算符方法签名类似 fn add(self, s: &str) -> String
    println!("{:?}", s3);    // s3实际获得了s1的所有权

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{} - {} - {}", s1, s2, s3);
    println!("{:?}", s);
    println!("{}", s1);    // 所有权没有转移

    let name = "林冠强";
    let s = &name[0..3];
    // let s = &name[0..4];    // thread 'main' panicked at 'byte index 4 is not a char boundary
    println!("{:?}", s);

    for c in name.chars() {
        println!("{}", c);
    }
    for b in name.bytes() {
        println!("{}", b);
    }
}
