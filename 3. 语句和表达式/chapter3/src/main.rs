fn f(a: bool, b: bool, c: bool) -> bool {
    (a == b) == c
}

fn main() {
    println!("{}", f(true, false, true));

    let num1: u8 = 0b_1010_1010;
    let num2: u8 = 0b_1111_0000;
    println!("{:08b}", !num1);
    println!("{:08b}", num1 & num2);
    println!("{:08b}", num1 | num2);
    println!("{:08b}", num1 ^ num2);
    println!("{:08b}", num1 << 4);
    println!("{:08b}", num1 >> 4);

    let x = 1;
    let mut y = 2;
    println!("{}", y); // 2
    let z = y = x;
    println!("{}", y); // 1
    println!("{:?}", z); // ()，因为赋值表达式y=x的类型是()

    let x: () = {
        println!("Hello.");
    };
    println!("{:?}", x); // ()
    let y: i32 = {
        println!("Hello.");
        5
    };
    println!("{}", y); // 5, 注意——5后面没有分号，整个语句块的类型就是i32

    let v = loop {
        break 10;
    };
    println!("{}", v);

    let v = loop { break };
    println!("{:?}", v);    // ()

    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }

    let array = &[1, 2, 3, 4, 5];
    for i in array {
        println!("The number is {}", i);
    }

    let array = [1, 2, 3, 4, 5];
    for i in &array {
        println!("The number is {}", i);
    }

    let array = [1, 2, 3, 4, 5];
    for i in array.iter() {
        println!("The number is {}", i);
    }
}