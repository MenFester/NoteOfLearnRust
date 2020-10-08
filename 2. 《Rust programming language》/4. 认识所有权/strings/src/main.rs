fn main() {
    let mut s = String::from("hello");    // 双冒号::运算符允许我们调用置于String命名空间下的特定from函数
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1;    // s1无效了，这里发生浅度拷贝，即变量移动
    println!("{}, world!", s2);    // 这里若使用s1则报错

    let s1 = String::from("hello");
    let s2 = s1.clone();    // 这里复制了堆上的数据
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;    // 存在栈中的类型执行复制，所以x继续持有变量
    println!("x = {}, y = {}", x, y);

    let bytes = s1.as_bytes();
    for (i, item) in bytes.iter().enumerate() {
        println!("{} => {}", i, item);
    }
}
