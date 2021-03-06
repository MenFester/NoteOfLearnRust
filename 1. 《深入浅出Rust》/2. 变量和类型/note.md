# 学习笔记

## 2.1 变量声明

* Rust变量必须先声明后使用，例如：` let variable : i32 = 100; `
* 局部变量声明一定是以关键字let开头
* 类型一定是跟在冒号后
* Rust的设计考虑了类型自动推导功能，因此类型后置的语法更合适
* let语句除局部变量声明，还具有pattern destruction（模式解构）的功能，例如：` let (mut a, mut b) = (1, 2) `
* Rust中声明的变量缺省是“只读”的，需要让变量是可写的，需要使用mut关键字。例如：` let mut x = 5; `。let处引入模式解构，把mut x视为一个组合（“模式”）
* Rust中，一般把声明的局部变量并初始化的语句称为“变量绑定”，强调的是“绑定”的含义
* Rust中，变量必须被合理初始化（不一定是在声明的同时就初始化，初始化时不需要变量是mut的，因为是声明而不是修改）之后才能被使用（利用unsafe做hack除外）。注意全局变量必须当场初始化，main函数里let声明的也是局部变量，只有static声明的才是全局变量
* 类型没有“默认构造函数”，变量没有“默认值”
* Rust里的合法标识符必须由数字、字母、下划线组成，且不能以数字开头
* raw identifier：特殊语法（如r#self），让用户可以以关键字作为普通标识符，这只是特殊情况下迫不得已的做法
* Rust里下划线是一个特殊标识符，在编译器内部它是被特殊处理的。不能在表达式中用下划线来作为普通变量使用。下划线表达的含义是“忽略这个变量绑定，后面不会再用到了”。在泛型的尖括号中用下划线，是将剩下的部分让编译器去推导
* 变量遮蔽：Rust允许在同一个代码中声明同样名字的变量。我们需要理解的是：一个“不可变绑定”依然是一个“变量”，没办法通过“不可变绑定”修改变量的值，但是重新使用可变绑定（通过变量遮蔽做到）之后，就可以通过“可变绑定”修改变量的值（此时如果没有其他引用指向这个变量，对这个变量的修改就是完全合法的，是符合“内存安全”的）
* 类型推导：可以从变量声明的当前语句中获取信息进行推导，还能通过上下文信息进行推导。Rust只允许“局部变量/全局变量”实现类型推导，函数签名等场景下是不允许的
* 类型别名：用type关键字给同一类型起个别名（type alias），例如：` type Age = u32; `。类型别名用在泛型场景：` type     Double<T> = (T , Vec<T>); `
* 静态变量：用static关键字声明静态变量。例如：` static GLOBAL: i32 = 0; `。static语句同样也是一个模式匹配，与let语句不同，static声明的变量的生命周期是整个程序，从启动到退出，它占用的内存空间不会在执行过程中回收（全局变量的内存不是分配在当前函数栈上，函数退出时并不会销毁全局变量占用的内存空间，程序退出才会回收）。这是Rust中唯一声明全局变量的方法
* 由于Rust非常注重内存安全，全局变量的使用有许多限制：
  * 全局变量必须在声明的时候马上初始化.
  * 全局变量的初始化必须是编译期可确定的常量，不能包括执行期才能确定的表达式、语句和函数调用
  * 带有mut修饰的全局变量，在使用的时候必须使用unsafe关键字
  * 禁止在声明static变量的时候调用普通函数，或者利用语句块调用其他非const代码。调用const fn是允许的，因为const fn是编译期执行的
  * Rust不允许用户在main函数之前或者之后执行自己的代码。所以，比较复杂的static变量的初始化一般需要用lazy方式，在第一次使用的时候初始化。如果需要使用比较复杂的全局变量初始化，推荐使用lazy_static库
* 常量：使用const声明的是常量而不是变量，因此一定不允许使用mut关键字修饰这个绑定。例如：` const GLOBAL: i32 = 0 `。
* 常量的初始化表达式也一定要是一个编译期常量。它与static变量的最大区别在于：编译器不一定会给常量分配内存空间，很可能被内联优化。以const声明常量不具备类似let语句的模式匹配功能

## 2.2 基本数据类型

* bool：它有两个值，true和false。操作符：
  * ！：取反
  * &&：逻辑与，带短路功能
  * ||：逻辑或，带短路功能
  * &：按位与，不带短路功能
  * |：按位或，不带短路功能
  * ^：按位异或，不带短路功能
* char：字符类型，可以描述任何一个符合Unicode标准的字符值（占内存空间4个字节）。单个字符字面量用单引号包围。字符字面量也可以用转义字符，例如：` '\x7f'，'\u{7FFF}' `
* Rust提供了单字节字符字面量来表示ASCII字符（只须占用1个字节的空间），使用字母b在字符或者字符串前面，存储在u8类型中。例如：` let r: &[u8; 14] = br#"hello \n world"#; `
* 整数类型：
  * 有符号：i8，无符号：u8
  * 有符号：i16，无符号：u16
  * 有符号：i32，无符号：u32
  * 有符号：i64，无符号：u64
  * 有符号：i128，无符号：u128
  * 有符号：isize，无符号：usize。占据的空间是不定的，与指针占据的空间一致，与所在的平台相关。
* 数字类型字面量表达可以有许多方式：
  * 0x开头代表十六进制
  * 0o开头代表八进制
  * 0b开头代表二进制 
* 在所有数字字面量中，可以在任何地方添加任意下划线，以方便阅读
* 字面量后面可以跟后缀，代表该数字的具体类型，从而省略显示类型标记
* 在Rust中，可以为任何一个类型添加方法。例如：整数类型有pow方法，甚至可以直接对整型字面量调用函数
* 对于整型，如果Rust编译器通过上下文无法分析出该变量的具体类型，则自动默认为i32类型
* 整数溢出：默认情况下，debug模式下编译器会自动插入整数溢出检查，一旦发生溢出，则会引发panic；在release模式下，不检查整数溢出，而是采用自动舍弃高位的方式（` rustc -O note.rs `）
* Rust编译器还提供独立的编译开关，可以设置溢出处理策略：` rustc -C overflow-checks=no note.rs ` 
* 在某些场景下，用户确实需要更精细的自主控制整数溢出的行为，可以调用标准库中的checked_*、saturating_*、wrapping_*系列函数。在安全性要求非常高的情况下，尽量用这几个方法替代默认的算术运算符来做数学运算
  * checked_*：返回的类型是Option<_>，当溢出的时候返回值是None
  * saturating_*：返回类型是整数，如果溢出则给出该类型可表示范围的“最大/最小”值
  * wrapping_*：系列函数则是直接抛弃已经溢出的最高位，将剩下的部分返回
* 标准库` std::num::Wrapping<T> `类型：重载了基本运算符，可以被当做普通整数使用，凡是被它包裹起来的整数，任何时候出现溢出都是截断行为
* 浮点类型：分部为f32和f64，浮点数的麻烦之处在于它不仅可以表达正常的数值，还可以表达不正常的数值。在标准库中` std::num::FpCategory `枚举了浮点数的可能状态
  * Zero：表示0值
  * Normal：表示正常状态的浮点数
  * SubNormal：浮点数表示精度比Normal状态下的精度低一点
  * Infinite：代表无穷大，例如：非0数除以0
  * NaN：代表“不是数字”，例如：0除以0。NaN不具备“全序”的特点。因为NaN存在，浮点数是不具备“全序关系”的
  * 浮点数的二进制表达方式：` x = (-1)^s * (1+M)*2^e `
    * s是符号位
    * M是尾数，是一个[0,1)范围内二进制表示的小数
    * e是指数
* 指针类型：  
  * `Box<T>`：指向类型T的，具有所有权的指针，有权释放内存
  * `&T`：指向类型T的借用指针，也称为引用，无权释放内存，无权写数据
  * `&mut T`：指向类型T的mut型借用指针，无权释放内存，有权写数据
  * `*const T`：指向类型T的只读裸指针，没有生命周期信息，无权写数据
  * `*mut T`：指向类型T的可读写裸指针，没有生命周期信息，有权写数据
* 标准库中还有一种封装起来的可以当做指针使用的类型，叫做“智能类型”
  * `Rc<T>`：指向类型T的引用计数指针，共享所有权，线程不安全
  * `Arc<T>`：指向类型T的原子型引用计数指针，共享所有权，线程安全
  * `Cow<'a, T>`：Clone-on-write，写时复制指针。可能是借用指针，也可能是具有所有权的指针
* 类型转换：Rust对不同类型之间的转换控制得非常严格，关键字as专门用于这样的类型转换（显示类型转换）。Rust设计者希望在发生类型转换的时候是显示地标记出来，防止隐藏的bug。关键字as不是随便可用的，它只允许编译器认为合理的类型转换，任意类型转换是不允许的。如果需要更复杂的类型转换，一般使用标准库的From Into等trait 

## 2.3 复合数据类型

* 复合数据类型可以在其他类型的基础上形成更复杂的组合关系
* tuple：元组类型，通过圆括号包含一组表达式构成。tuple内的元素没有名字，是把几个类型组合到一起的最简单的方式。
  * 如果元组只包含一个元素，应该在后面添加一个逗号以区分括号表达式和元组
  * 访问元组内部有两种方式：“模式匹配”和“数字索引” 
  * 元组内部也可以一个元素也没有，这个类型叫unit（单元类型）。例如：` let empty: () = (); `，占用0内存空间
  * std::mem::size_of函数可以计算一个类型所占用的内存空间
* struct：与元组类似，可以把多个类型组合到一起，作为新的类型。结构体与元组的区别在于，它的每个元素都有自己的名字
  * 每个元素之间采用逗号分开，最后一个逗号可以省略不写
  * 不能使用自动类型推导功能，必须显示指定
  * struct类型的初始化语法类似于json的语法，使用“成员-冒号-值”格式
  * Rust允许struct类型的初始化使用一种简化的写法：如果局部变量名称和成员变量名称恰好一致，可以省略冒号
  * 访问结构体内部的元素，也是使用“点”加变量名的方式，也可以使用模式匹配
  * Rust设计了一个语法糖:` ..expr `，只能放在初始化表达式中
  * struct内部成员可以是空：` struct Foo1;    struct Foo2();    struct Foo3() `
* tuple struct：就像tuple和struct的混合，区别在于tuple struct有名字，而它们的成员没有名字。例如：` struct Color(i32, i32, i32); `。可以把tuple struct想象为成员名字为0 、1、 2...的struct
* tuple、struct、tuple struct起的作用都是把几个不同类型的成员打包组合成一个类型，它们的区别：

| 类型名称 | tuple | struct | tuple struct |
|:---:|:---:|:---:|:---:|:---:|
|语法|没有名字圆括号|名字加大括号|名字加圆括号|
|类型名字|没有单独的名字|有单独的名字|有单独的名字|
|成员名字|没有单独的名字|有单独的名字|没有单独的名字|

* tuple struct有一个特别有用的场景：当它只包含一个元素的时候，就是所谓的newtype idiom。通过type关键字，我们可以创建一个新的类型名称，但是这个类型不是全新的类型，而是一个具体类型的别名。使用tuple struct做包装，则是创造了一个全新的类型，它跟被包装类型不能发生隐式类型转换，可以拥有不同的方法，满足不同的trait
* enum：Rust中的enum可以为每个成员指定附属的类型信息。
  * 如果说tuple、struct、tuple struct在Rust中代表多个类型的“与”关系，enum在类型在Rust中代表的就是多个类型的或关系。
  * Rust的enum中的每个元素的定义语法与struct的定义语法类似。可以像空结构体一样，不指定它的类型;也可以像tuple struct一样，用圆括号加无名成员;还可以像正常结构体一样，用大括号加带名字的成员
  * enum产生新的类型
  * 要使用enum，一般要用到“模式匹配”
  * Rust的enum类型的变量需要区分它里面的数据究竟是哪种变体，所以它包含了一个内部的“tag标记”来描述当前变量属于哪种类型。这个标记对用户是不可见的。可以手动指定每个变体自己的标记值
  * Rust里面也支持union类型，这个类型与C语言中的union完全一致。但在Rust里面，读取它内部的值被认为是unsafe行为，一般情况下我们不使用这种类型。它存在的主要目的是为了方便与C语言进行交互
  * 在Rust中，enum和struct为内部成员创建了新的名字空间。如果要访问内部成员，可以使用::符号
  * 
  * Rust标准库中有一个极其常用的enum类型`Option<T>`，由于它实在是太常用，标准库将Option以及它的成员Some、None 都加入到了Prelude中
  * Rust的enum实际上是一种代数类型系统(Algebraic Data Type， ADT)。enum内部的variant只是一个名字而已，我们可以把enum内部的variant当成一个函数使用，恰好我们还可以将这个名字作为类型构造器使用
* 类型递归定义：Rust里面的复合数据类型是允许递归定义的。比如struct里面嵌套同样的struct类型，但是直接嵌套是不行的。直接使用类型递归定义的问题在于，当编译器计算递归定义的类型大小时实数范围无解。解决办法很简单，用指针间接引用就可以了，因为指针的大小是固定的