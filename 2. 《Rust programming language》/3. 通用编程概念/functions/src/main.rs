fn main() {
    println!("Hello, world!");

    another_function1();
    another_function2(5);
    another_function3(6, 7);

    let x = 5;    // 语句，没有返回值
    println!("The value of x is: {}", x);
    let y = {    // 语句块表达式
        let x = 3;
        x + 1    // 表达式，结尾没有分号，表达式的结果就是语句块的返回值。如果有分号就是语句，没有返回值
    };
    println!("The value of y is: {}", y);
    
    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x + 1 is: {}", x);
}

fn another_function1() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

fn another_function3(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1    // 加上分号结尾，则编译报错
}