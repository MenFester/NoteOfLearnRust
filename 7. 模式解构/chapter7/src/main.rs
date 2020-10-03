struct T1 (i32, char);

struct T2 {
    item1: T1,
    item2: bool,
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
}
