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
}
