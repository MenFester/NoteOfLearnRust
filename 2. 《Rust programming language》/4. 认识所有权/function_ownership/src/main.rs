fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s);    // s的所有权被move进函数，这里s不再持有变量
    
    let x = 5;
    makes_copy(x);
    println!("{}", x);    // 整数类型是复制进函数，所以这里x依然持有变量

    let s1 = gives_ownership();
    println!("s1 = {}", s1);
    let s2 = String::from("hello from s2");
    let s3 = takes_and_gives_back(s2);
    println!("s3 = {}", s3);
}    // s3在这里离开作用域并被销毁；由于s2已经移动了，所以不会在离开作用域时发生任何事情；s1最后离开作用域并被销毁

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}    // some_string在这里离开作用域，drop函数被自动调用

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}    // some_integer在这里离开了作用域，没有什么特别的事情发生

fn gives_ownership() -> String {
    let some_string = String::from("hello to s1");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
