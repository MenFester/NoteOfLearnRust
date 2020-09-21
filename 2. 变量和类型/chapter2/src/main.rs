fn test(condition: bool) {
    let x: i32;
    if condition {
        x = 1; // 初始化，不需要x是mut的
        println!("{}", x);
    }
}

fn arithmetic(m: i8, n: i8) {
    println!("{}", m + n); // 有溢出风险
}

struct Point {
    x: i32,
    y: i32,
}

struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

fn default() -> Point3d {
    Point3d { x: 0, y: 0, z: 0 }
}

struct T1 {
    v: i32,
}

struct T2(i32);

enum Number {
    Int(i32),
    Float(f32),
}

fn read_num(num: &Number) {
    match num {
        &Number::Int(value) => println!("Integre {}", value),
        &Number::Float(value) => println!("Float {}", value),
    }
}

enum Animal {
    dog = 1,
    cat = 200,
    tiger,
}

fn main() {
    test(true);

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let v = v; // 用变量遮蔽修改变量绑定的可变性
    for i in &v {
        println!("{}", i);
    }

    let player_scores = [("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19)];
    let players: Vec<_> = player_scores // Vec动态数组包含类型不清楚，在尖括号中用下划线代替
        .iter()
        .map(|&(player, _score)| player)
        .collect();
    println!("{:?}", players);

    let x;
    let y = 1_i32;
    x = 2_i32;
    println!("{} {}", x, y);
    static G1: i32 = 3;
    println!("{}", G1);
    static mut G2: i32 = 4;
    unsafe {
        // 可变全局变量无论读写都必须使用unsafe修饰
        G2 = 5;
        println!("{}", G2);
    }

    let x: i32 = 9;
    println!("9 power 3 = {}", x.pow(3));
    println!("9 power 4 = {}", 9_i32.pow(4)); // 直接对整型字面量调用函数

    let m: i8 = 120;
    let n: i8 = 120;
    // arithmetic(m, n); // rustc -O 后不报panic

    let i = 100_i8;
    println!("checked {:?}", i.checked_add(i));
    println!("saturating {:?}", i.saturating_add(i));
    println!("wrapping {:?}", i.wrapping_add(i));

    let p = (1_i32, 2_i32);
    let (a, b) = p;
    let x = p.0;
    let y = p.1;
    println!("{} {} {} {}", a, b, x, y);

    println!("size of i8 {}", std::mem::size_of::<i8>());
    println!("size of char {}", std::mem::size_of::<char>());
    println!("size of '()' {}", std::mem::size_of::<()>());

    let p = Point { x: 0, y: 0 };
    println!("Point is at {} {}", p.x, p.y);

    let x = 10;
    let y = 20;
    let p = Point { x, y };
    println!("Point is at {} {}", p.x, p.y);

    let p = Point { x: 0, y: 0 };
    let Point { x: px, y: py } = p;
    println!("Point is at {} {}", px, py);
    let Point { x, y } = p;
    println!("Point is at {} {}", x, y);

    let origin = Point3d { x: 5, ..default() };
    let point = Point3d {
        z: 1,
        x: 2,
        ..origin // 不覆盖已有的x、z
    };
    println!("origin at {} {} {}", origin.x, origin.y, origin.z);
    println!("potint at {} {} {}", point.x, point.y, point.z);

    let v1 = T1 { v: 1 };
    let v2 = T2(1);
    let v3 = T2 { 0: 1 };
    println!("{}", v1.v);
    println!("{}", v2.0);
    println!("{}", v3.0);

    let n: Number = Number::Int(10);
    read_num(&n);
    let n: Number = Number::Float(3.14);
    read_num(&n);

    let x = Animal::tiger as isize;
    println!("{}", x);    // 201
}
