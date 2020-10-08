fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
/*
    --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: make this binding mutable: `mut x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

    x = 6;
    println!("The value of x is: {}", x);
*/
    let mut x = 5;    // 屏蔽同名变量
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    let f1 = 2.0;    // 推导为f64
    let f2: f32 = 3.0;    // f32
    println!("The value of f1 is: {}", f1);
    println!("The value of f2 is: {}", f2);

    let c = 'z';
    println!("The value of c is: {}", c);

    let tup: (i32, f64, u8) = (500, 6.4, 1);    // 不必要的元组类型注解
    println!("The tuple is: {:?}", tup);
    let tup = (500, 6.4, 1);
    let (_x, y , _z) = tup;    // 模式匹配、解构元组；x、z使用下划线开头使没有使用的变量编译不报错
    println!("The value of y is: {}", y);
    let five_hundred = tup.0;
    let six_point_five = tup.1;
    let one = tup.2;
    println!("{}, {}, {}", five_hundred, six_point_five, one);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The array is: {:?}", a);
    println!("The elemet a[3] is: {}", a[3]);

    let a = [3; 5];    // the array is: [3, 3, 3, 3, 3]
    println!("The array is: {:?}", a);
}