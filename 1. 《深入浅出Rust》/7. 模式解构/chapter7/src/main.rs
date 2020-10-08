struct T1 (i32, char);

struct T2 {
    item1: T1,
    item2: bool,
}

enum Direction {
    East, West, South, North
}

fn print_direction(x: Direction) {
    match x {    // exhaustive，如果删除任何一个分支，都会编译报错
        Direction::East => {
            println!("East");
        }
        Direction::West => {
            println!("West");
        }
        Direction::South => {
            println!("South");
        }
        Direction::North => {
            println!("North");
        }
    }
}

fn direction_to_int(x: Direction) -> i32 {
    match x {
        Direction::East => 10,    // 没有大括号的分支用逗号隔开
        Direction::West => 20,
        Direction::South => 30,
        Direction::North => 40,
    }
}

struct P(f32, f32, f32);

fn calc(arg: &P) -> f32 {
    let P(x, _, y) = arg;
    x * x + y * y
}

fn calc2(P(x, _, y): &P) ->f32 {
    x * x + y * y
}

fn category(x: i32) {
    match x {
        -1 => println!("negative"),
        0 => println!("zero"),
        1 => println!("positive"),
        _ => println!("error"),
    }
}

fn category2(x: i32) {
    match x {
        -1 | 1 => println!("true"),
        0 => println!("false"),
        _ => println!("error"),
    }
}

enum OptionalInt {
    Value(i32),
    Missing,
}

fn deep_match(v: Option<Option<i32>>) -> Option<i32> {
    match v {
        Some(r @ Some(1 ..= 10)) => r,
        _ => None,
    }
}

fn do_something_with(x: i32) {
    println!("this is a number: {}", x);
}

struct T {
    item1: char,
    item2: bool,
}

fn test(T{item1: arg1, item2: arg2} : T) {
    println!("{} {}", arg1, arg2);
}

fn main() {
    let tuple = (1_i32, false, 3f32);
    let (head, center, tail) = tuple;
    println!("{} {} {}", head, center, tail);

    let x = T2 {
        item1: T1 (0, 'A'),
        item2: false,
    };
    let T2 {
        item1: T1 (value1, value2),
        item2: value3,
    } = x ;
    println!("{} {} {}", value1, value2, value3);

    let x = Direction::West;
    print_direction(x);

    let t = P(1.0, 2.0, 3.0);
    println!("{}", calc(&t));
    println!("{}", calc2(&t));

    let x = (1, 2, 3);
    let (a, ..) = x;
    println!("{}", a);
    let (a, .., b) = x;    // .. 省略中间部分元素
    println!("{} {}", a, b);

    let x = Direction::East;
    let s = direction_to_int(x);
    println!("{}", s);

    let x = 1;
    category(x);
    category2(x);
    let x = 100;
    category(x);
    category2(x);

    let x = 'x';
    match x {
        'a' ..= 'z' => println!("lowercase"),
        'A' ..= 'Z' => println!("uppercase"),
        _ => println!("something else"),
    }

    let x = OptionalInt::Value(5);
    match x {
        OptionalInt::Value(i) if i > 5 => println!("Got an int bigger than five!"),    // 如果Value(6)，只执行本分支
        OptionalInt::Value(..) => println!("Got an int!"),
        OptionalInt::Missing => println!("No such luck"),
    }

    let x = 1;
    match x {
        e @ 1 ..= 5 => println!("got a range element {}", e),
        _ => println!("anything"),
    }

    let x = 5;
    match x {
        e @ 1 ..= 5 | e @ 8 ..= 10 => println!("got a range element {}", e),
        _ => println!("anything"),
    }

    let x = Some(Some(5));
    println!("{:?}", deep_match(x));
    let y = Some(Some(100));
    println!("{:?}", deep_match(y));

    let mut v = vec![1i32, 2, 3];
    v = vec![4i32, 5, 6];
    // v = vec![1.0f32, 2, 3];    报错

    let optVal = Some(3i32);

    if let Some(i) = optVal {
        do_something_with(i);
    }

    let x = T {
        item1: 'A',
        item2: false,
    };
    test(x);
}
