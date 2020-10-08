fn main() {
    let t1 = Coin::Penny;
    println!("{}", value_in_cents(t1));

    let t2 = Coin::Nickel;
    println!("{}", value_in_cents(t2));

    let t3 = Coin2::Quarter(UsState::Alaska);
    println!("{}", value_in_cents2(t3));

    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
    let none = plus_one(None);
    println!("{:?}", none);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => println!("other!"),
    }

    let some_u8_value2 = Some(3u8);
    if let Some(3) = some_u8_value2 {
        println!("three");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => {
            println!("Lucky Nickel!");
            5
        },
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents2(coin: Coin2) -> u32 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}