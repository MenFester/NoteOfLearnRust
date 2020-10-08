fn main() {
    /*
    loop {
        println!("again!");
    }
    */

    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;    // 这里会返回值，注意用了分号结尾
        }
    };
    println!("The result is: {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {    // rev方法用来翻转Range生成的序列
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
