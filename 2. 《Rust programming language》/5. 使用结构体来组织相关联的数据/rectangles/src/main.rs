fn main() {
    // 第一种：
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    // 第二种：使用元组，但是容易混淆
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    // 第三种：使用结构体重构
    let rect2 = Rectangle { width: 30, height:50 };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );
    println!("{:?}", rect2);
    println!("rect2 is {:#?}", rect2);    // 输出更易读一些

    let rect3 = Rectangle { width: 10, height:40 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect3.area()
    );

    println!("Can rect2 hold rect3 ? {}", rect2.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("sq is {:#?}", sq);
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {    // 元组使输入的参数结构化
    dimensions.0 * dimensions.1    // 程序变得难以阅读了
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {    // 接收一个不可变借用参数
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {    // 无须重复编写self的类型
        self.width * self.height
    }
}

impl Rectangle {    // 一个结构体可以对应多个impl块
    fn can_hold(&self, other: &Rectangle) -> bool {    // 方法可以有多个参数
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {    // 关联函数
        Rectangle { width: size, height: size }
    }
}