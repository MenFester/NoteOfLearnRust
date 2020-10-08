fn main() {
    let mut v1: Vec<i32> = Vec::new();    // 需要显示地增加类型标记，因为没有插入值Rust无法自动推导
    println!("{:?}", v1);

    let v2 = vec![1, 3, 4];    // 带初始化值，所以可以类型推导
    println!("v2 = {:?}", v2);

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);
    println!("v1 = {:?}", v1);

    let v3 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v3[2];
    println!("The third element is {}", third);
    match v3.get(2) {    // 返回一个Option<&T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v4 = vec![1, 2, 3, 4, 5];
    let first = &v4[0];
    v4.push(6);
    println!("The first element is: {:?}", v4);
    // println!("{:?}", first);    // 在同一个作用域中同时使用变引用和不可变引用，报错

    for i in &v4 {
        println!("{}", i);
    }

    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        println!("before: {}", *i);    // 使用*i 和 i结果一致
        *i += 50;    // 运算符*来获得i绑定的值
        println!("after: {}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
    for i in &row {    // 从枚举中取值
        match i {
            SpreadsheetCell::Int(x) => println!("{:?}", x),
            SpreadsheetCell::Float(x) => println!("{}", x),
            SpreadsheetCell::Text(s) => println!("{}", s),
        }
    }
    println!("{:?}", row);    // 如果for语句不使用&row，这里将报错
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
