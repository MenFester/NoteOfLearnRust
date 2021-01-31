use std::collections::HashMap;

fn req_status() -> u32 {
    404
}

fn silly_sub(a: i32, b: i32) -> i32 {
    let mut result = 0;
    'increment: loop {
        if result == a {
            let mut dec = b;
            'decrement: loop {
                if dec == 0 {
                    break 'increment
                } else {
                    result -= 1;
                    dec -= 1;
                }
            }
        } else {
            result += 1;
        }
    }
    result
}

struct Color(u8, u8, u8);

struct Player {
    name: String,
    iq: u8,
    friend: u8,
    score: u16
}

impl Player {
    fn with_name(name: &str) -> Player {
        Player {
            name: name.to_string(),
            iq: 100,
            friend: 100,
            score: 120
        }
    }
    
    fn get_friend(&self) -> u8 {
        self.friend
    }

    fn set_friend(&mut self, count: u8) {
        self.friend = count;
    }
}

fn bump_player_score(mut player: Player, score: u16) {
    player.score += score;
    println!("Updated player stats:");
    println!("Name: {}", player.name);
    println!("IQ: {}", player.iq);
    println!("Friends: {}", player.friend);
    println!("Score: {}", player.score);
}

#[derive(Debug)]
enum Direction {
    N,
    E,
    S,
    W
}

enum PlayerDirection {
    Move {    // 类C结构体
        direction: Direction,
        speed: u8
    },
    Wait,
    Attack(Direction)    // 元组结构体
}

enum PaymentMode {
    Debit,
    Credit,
    Paypal
}

fn pay_by_credit(amt: u64) {
    println!("Processing credit payment of {}", amt);
}

fn pay_by_debit(amt: u64) {
    println!("Processing debit payment of {}", amt);
}

fn paypal_redirect(amt: u64) {
    println!("Redirecting to paypal for amount: {}", amt);
}

impl PaymentMode {
    fn pay(&self, amount: u64) {
        match self {
            PaymentMode::Debit => pay_by_debit(amount),
            PaymentMode::Credit => pay_by_credit(amount),
            PaymentMode::Paypal => paypal_redirect(amount)
        }
    }
}

fn get_saved_payment_mode() -> PaymentMode {
    PaymentMode::Debit
}

fn main() {
    // if-else as a expression

    let result = if 1 == 2 {
        "Wait, What?"
    } else {
        "Rust makes sense"
    };

    println!("You know what? {}", result);

    let result = if 1 == 2 {
        "Wait, What?";
    } else {
        "Rust makes sense";
    };

    println!("You know what? {:?}", result);

    // match

    let status = req_status();
    match status {
        200 => println!("Success!"),
        404 => {
            println!("oh!");
            println!("Not Found");
        }
        other => {
            println!("Request failed with code: {}", other);
        }
    }

    // break label

    let a = 10;
    let b = 4;
    let result = silly_sub(a, b);
    println!("{} minus {} is {}", a, b, result);

    // for

    println!("Normal ranges: ");
    for i in 0..10 {
        print!("{}, ", i);
    }

    println!();
    println!("Inclusive ranges: ");
    for i in 0..=10 {
        print!("{}, ", i);
    }

    // tuple struct

    let white = Color(255, 255, 255);
    let red = white.0;
    let green = white.1;
    let blue = white.2;

    println!("Red value: {}", red);
    println!("Green value: {}", green);
    println!("Blue value: {}", blue);

    let orange = Color(255, 165, 0);
    let Color(r, g, b) = orange;
    println!("R: {}, G: {}, B: {} (orange)", r, g, b);

    let Color(r, _, b) = orange;
    println!("({}-_-{})", r, b);

    // 类C结构体
    let name = "Alice".to_string();
    let player = Player {
        name,
        iq: 171,
        friend: 134,
        score: 1129
    };
    bump_player_score(player, 120);

    // enum

    let simulated_player_action = PlayerDirection::Move {
        direction: Direction::N,
        speed: 2
    };

    match simulated_player_action {
        PlayerDirection::Wait => println!("Player wants to wait."),
        PlayerDirection::Move {direction, speed} => {
            println!("Player wants to move in direction {:?} with speed {}", direction, speed);
        },
        PlayerDirection::Attack(direction) => {
            println!("Player wants to attack direction {:?}", direction);
        }
    };

    // impl for struct

    let mut player = Player::with_name("Dave");
    player.set_friend(23);
    println!("{}'s friends count: {}", player.name, player.get_friend());
    let _ = Player::get_friend(&player);    // 另一种调用实例方法的方式

    // impl for enum

    let payment_mode = get_saved_payment_mode();
    payment_mode.pay(512);

    // arrays

    let numbers: [u8; 10] = [1, 2, 3, 4, 5, 7, 8, 9, 10, 11];
    let floats = [0.1f64, 0.2, 0.3];

    println!("Number: {}", numbers[5]);
    println!("Float: {}", floats[2]);

    // tuple

    let num_and_str: (u8, &str) = (40, "Have a good day!");
    println!("{:?}", num_and_str);
    let (num, string) = num_and_str;
    println!("From tuple: Number: {}, String: {}", num, string);

    // vec
    
    let mut numbers_vec: Vec<u8> = Vec::new();
    numbers_vec.push(1);
    numbers_vec.push(2);

    let mut vec_with_macro = vec![];
    vec_with_macro.push(2);

    let _ = vec_with_macro.pop();

    let message = if numbers_vec == vec_with_macro {
        "The are equal"
    } else {
        "Nah! They look different to me"
    };

    println!("{} : {:?} {:?}", message, numbers_vec, vec_with_macro);

    // HashMap

    let mut fruits = HashMap::new();
    fruits.insert("apple", 3);
    fruits.insert("mango", 6);
    fruits.insert("orange", 2);
    fruits.insert("avocado", 7);

    for (k, v) in &fruits {
        println!("I got {} {}", v, k);
    }
    
    fruits.remove("orange");
    let old_avocado = fruits["avocado"];
    fruits.insert("avocado", old_avocado + 5);
    println!("\n I now have {} avocados", fruits["avocado"]);

    // slice

    let mut numbers: [u8; 4] = [1, 2, 3, 4];
    {
        let all: &[u8] = &numbers[..];
        println!("All of them: {:?}", all);
    }

    {    // 额外花括号，用于隔离从不可变引用中获取切片的可变引用的代码
        let first_two: &mut [u8] = &mut numbers[0..2];
        first_two[0] = 100;
        first_two[1] = 99;
        println!("Look ma! I can modify through slices: {:?}", numbers);
    }
}
