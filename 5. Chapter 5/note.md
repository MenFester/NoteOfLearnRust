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

## 5.2 静态方法

* 没有receiver的方法（第一个参数不是小写self参数，即便第一个参数类型是Self相关类型的方法）称为静态方法，用Type::FunctionName()的方式调用，不能用小数点的语法调用函数
* trait中也可以定义静态函数
* Rust中没有“构造函数”的概念，Default trait实际上可以看做一个针对无参数构造函数的统一抽象

## 5.3 扩展方法

* 可以利用trait给其他的类型添加成员方法，哪怕类型不是自己写的
* 在声明trait和impl trait的时候，Rust规定了Coherence Rule（一致性规则）或称为Orphan  Rule（孤儿规则）：impl块要么与trait的声明在同一个crate中，要么与类型的声明在同一个crate中。同理，如果是匿名impl，那么这个impl块必须与类型本身存在于同一个crate中
* 许多初学者会用自带GC的语言中的“Interface”、抽象基类来理解trait这个概念，但是实际上它们有很大的不同：
  * Rust是一种用户可以对内存有精确控制能力的强类型语言。我们可以自由指定一个变量是在栈里面，还是在堆里面，变量和指针也是不同的类型。类型是有大小（Size）的。有些类型的大小是在编译阶段可以确定的，有些类型的大小是编译阶段无法确定的。目前版本的Rust规定，在函数参数传递、返回值传递等地方，都要求这个类型在编译阶段有确定的大小。否则，编译器就不知道该如何生成代码了
  * 而trait本身既不是具体类型，也不是指针类型，它只是定义了针对类型的、抽象的“约束”。不同的类型可以实现同一个trait，满足同一个trait的类型可能具有不同的大小。因此，trait在编译阶段没有固定大小，目前我们不能直接使用trait作为实例变量、参数、返回值

## 5.4 完整函数调用语法

* UFCS（universal function call syntax），允许精确地指定想要调用的是哪个函数、方法（包括成员方法和静态方法），其他一切函数调用语法都是它的某种简略形式。具体写法为` <T as TraitName>::item `
* 通过小数点语法调用方法，有一个“隐藏”的“取引用”步骤，例如me.start()真正传递给start()方法的参数是&me而不是me
* 如果用UFCS调用方法，就不能让编译器帮我们自动取引用了，必须手动写清楚
* 成员方法和普通函数其实没什么本质区别

## 5.5 trait约束和继承

* Rust的trait的另外一个大用处是，作为泛型约束使用
* 泛型约束既是对实现部分的约束，也是对调用部分的约束
* trait允许继承，满足Derived的类型，必然也满足Base trait。所以，我们在针对一个具体类型impl Derived的时候，编译器也会要求我们同时impl Base
* 在编译器的眼中，trait Derived：Base{}等同于trait Derived where Self：Base{}

## 5.6 Derive