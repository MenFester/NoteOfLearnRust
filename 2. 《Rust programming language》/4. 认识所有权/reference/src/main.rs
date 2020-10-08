fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);    // &s1允许我们在不转移所有权的前提下，创建一个指向s1值的引用
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("After change, s = {}", s);

    let r1 = &s;
    println!("r1 = {}", r1);
    let r2 = &mut s;
    println!("r2 = {}", r2);
    // println!("r2 = {}  r1 = {}", r2, r1);    // 有这句，cargo编译报错：immutable borrow later used here
    let r3 = &mut s;
    // println!("{} {}", r2, r3);    // 有这句，cargo编译报错: first borrow later used here
    r3.push_str("...ok?");
    println!("r3 = {}", r3);

    let mut o = String::from("hello");
    let _o1 = &o;
    let _o2 = &o;
    let _o3 = &mut o;    // 不报错
    let _o4 = &mut o;    // 不报错

    // let reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {    // 函数签名中的&用来表明参数s的类型是一个引用
    s.len()
}

fn change(some_thing: &mut String) {
    some_thing.push_str(", world.");
}

/*
fn dangle() -> &String {    // 报错：expected named lifetime parameter
    let s = String::from("hello");
    &s
}
*/

