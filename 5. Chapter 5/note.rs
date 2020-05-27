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

fn main() {
    let c = Circle { radius: 2_f64 };
    println!("The area is {}", c.area1());
    println!("The radius of c is {}", c.get_radius());
    // c.area2()报错
    let b = Box::new(Circle { radius: 4_f64 });
    println!("The area is {}", b.area2());

    let b2 = Box::new(Circle { radius: 4_f64 }) as Box<dyn Round>;
    println!("The area is {}", b2.area3())
}
