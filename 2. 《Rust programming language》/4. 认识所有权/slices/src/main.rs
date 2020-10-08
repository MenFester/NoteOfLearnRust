fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello = {}; world = {}", hello, world);

    let slice = &s[0..2];
    println!("{}", slice);
    let slice = &s[..2];    // 等价的语法
    println!("{}", slice);

    let len = s.len();
    let slice = &s[3..len];
    println!("{}", slice);
    let slice = &s[3..];    // 等价的语法
    println!("{}", slice);

    let slice = &s[0..len];
    println!("{}", slice);
    let slice = &s[..];    // 等价的语法
    println!("{}", slice);

    // let mut test_string = String::from("hello world");
    let test_string = String::from("hello world");
    let word = first_word(&test_string);
    // test_string.clear();    // 报错，上一个语句拥有了某个变量不可变引用，本语句就无法同时取得该变量的可变引用
    println!("the first word is: {}", word);

    let my_string = String::from("hello world");
    let word = first_word2(&my_string[..]);
    println!("the first word-2 is: {}", word);
    
    let my_string_literal = "hello world";
    let word = first_word2(&my_string_literal[..]);
    println!("the first word-3 is: {}", word);

    let word = first_word2(my_string_literal);    // 字符串字面量本身就是切片，可以直接传递
    println!("the first word-4 is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word2(s: &str) -> &str {    // 使用字符串切片作为参数，使API更加通用
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}