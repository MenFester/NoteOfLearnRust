fn add1(t: (i32, i32)) -> i32 {
    t.0 + t.1
}

fn add2((x, y): (i32, i32)) -> i32 {
    x + y
}

fn test_inner() {
    // 函数内定义其他item
    static INNER_STATIC: i64 = 42;

    fn internal_incr(x: i64) -> i64 {
        x + 1
    }

    struct InnerTemp(i64);

    impl InnerTemp {
        fn incr(&mut self) {
            self.0 = internal_incr(self.0);
        }
    }

    let mut t = InnerTemp(INNER_STATIC);
    t.incr();
    println!("{}", t.0);
}

fn main() {
    let p = (1, 3);
    let func = add2;
    println!("evaluation output {}", func(p));
    println!("evaluation output {}", add1(p));

    let mut func1 = add1 as fn((i32, i32)) -> i32; // 就像把函数签名当做可以被其他函数类型的通用类型
                                                   // 或者 let mut func1: fn(i32, i32) -> i32 = add1;
    println!("evaluation output {}", func1(p));
    func1 = add2;
    println!("evaluation output {}", func1(p));

    test_inner();

    for arg in std::env::args() {
        println!("Arg: {}", arg);
    }

    std::process::exit(0);
}
