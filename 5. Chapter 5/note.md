# 学习笔记

## 5.1 成员方法

* trail中可以包含：函数、常量、类型等
* 所有的trail中都有一个隐藏的类型Self，代表当前这个实现了此trail的具体类型。
* trail中定义的函数，称为关联函数（associated function）。
* 关联函数的第一个参数如果是Self相关的类型，这个参数称为接收者（receiver）。具有receiver参数的函数，称为“方法”。
* 方法通过变量实例使用点号来调用
* 没有receiver的关联函数称为“静态函数（static function）”，通过类型加双冒号::的方式调用
* Rust中，函数和方法没有本质区别
* Self是类型名、self是变量名，两者都是关键字
* self参数同样可以指定类型，不同类型影响方法调用的形式:
  * self: Self，简写为self
  * self: &Self，简写为&self
  * self: &mut Self，简写为&mut self
  * self: Box<Self>
* 可以为某些类型实现（impl）这个trait
* 针对一个类型，我们可以直接对它impl来增加成员方法，无须trait名字。可以看做是为类型impl了一个匿名trait，定义了类型的“内在方法”（inherent methods）
* trait中可以包含方法的默认实现，具体类型实现时候就可以选择不重写
* impl的对象甚至可以是trait
* 