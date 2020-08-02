trait Shape {
    // 类似接口，但又有方法定义的内涵
    fn area1(self: &Self) -> f64; // 方法签名
    fn area2(self: Box<Self>) -> f64;
}

trait Shape2 {
    fn area3(self: &Self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    // 类似实现接口，但也是方法实现
    fn area1(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn area2(self: Box<Self>) -> f64 {
        // 大写Self类型是Circle，小写self的类型是Box<Circle>
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Circle {
    fn get_radius(&self) -> f64 {
        self.radius
    }
}

trait Round {
    fn get_radius3(&self) -> f64;
}

impl Round for Circle {
    fn get_radius3(&self) -> f64 {
        self.radius
    }
}

impl Shape2 for dyn Round {
    // 看做接口功能传递或装饰，为所有满足Round的具体类型增加一个成员方法
    fn area3(&self) -> f64 {
        std::f64::consts::PI * self.get_radius3() * self.get_radius3()
    }
}

struct T(i32);

impl T {
    fn func(this: &Self) {
        println!("value {}", this.0)
    }
}

trait Double {
    fn double(&self) -> Self;
}

impl Double for i32 {
    fn double(&self) -> i32 {
        *self * 2
    }
}

trait Cook {
    fn start(&self);
}

trait Wash {
    fn start(&self);
}

struct Chef;

impl Cook for Chef {
    fn start(&self) {
        println!("Cook::start");
    }
}

impl Wash for Chef {
    fn start(&self) {
        println!("Wash::start");
    }
}

use std::fmt::Debug;

fn my_print<T: Debug>(x: T) {
    // my_print函数引入了一个泛型参数T，所以它的参数不是一个具体类型，而是一组类型
    // 要求这个T类型实现Debug这个trait
    println!("The value is {:?}.", x);
}

fn my_print2<T>(x: T)
where
    T: Debug,
{
    println!("The value is {:?}.", x);
}

fn main() {
    let c = Circle { radius: 2_f64 };
    println!("The area is {}", c.area1());
    println!("The radius of c is {}", c.get_radius());
    // c.area2()报错
    let b = Box::new(Circle { radius: 4_f64 });
    println!("The area is {}", b.area2());

    let b2 = Box::new(Circle { radius: 4_f64 }) as Box<dyn Round>;
    println!("The area is {}", b2.area3());

    let x = T(42);
    T::func(&x);

    let x: i32 = 10.double();
    println!("{}", x);

    let me = Chef;
    <dyn Cook>::start(&me);
    <Chef as Wash>::start(&me);

    my_print("China"); // 凡是满足Debug约束的类型都可以是这个函数的参数
    my_print(41_i32);
    my_print(true);
    my_print(['a', 'b', 'c']);

    my_print2("China");
    my_print2(41_i32);
    my_print2(true);
    my_print2(['a', 'b', 'c']);
}
