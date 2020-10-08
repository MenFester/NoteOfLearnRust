fn main() {
    println!("Hello, world!");

    let number = 3;
    if number < 5 {
        println!("condiction was true");
    } else {
        println!("condiction was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condiction = true;
    let number = if condiction {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);
}
