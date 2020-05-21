# 学习笔记

## 2.1 变量声明

* Rust变量必须先声明后使用，例如：` let variable : i32 = 100; `
* 局部变量声明一定是以关键字let开头
* 类型一定是跟在冒号后
* Rust的设计考虑了类型自动推导功能，因此类型后置的语法更合适
* let语句除局部变量声明，还具有pattern destruction（模式解构）的功能
* Rust中声明的变量缺省是“只读”的，需要让变量是可写的，需要使用mut关键字。例如：` let mut x = 5; `。let处引入模式解构，把mut x视为一个模式。
* Rust中，一般把声明的局部变量并初始化的语句称为“变量绑定”，强调的是“绑定”的含义
* Rust中，变量必须被合理初始化（不一定是在声明的同时就初始化）之后才能被使用（利用unsafe做hack除外）
* 类型没有“默认构造函数”，变量没有“默认值”
* raw identifier：特殊语法（如r#self），让用户可以以关键字作为普通标识符，这只是特殊情况下迫不得已的做法
* Rust里下划线是一个特殊标识符，在编译器内部它是被特殊处理的。不能在表达式中用下划线来作为普通变量使用。下划线表达的含义是“忽略这个变量绑定，后面不会再用到了”
* 变量遮蔽：Rust允许在同一个代码中声明同样名字的变量。我们需要理解的是：一个“不可变绑定”依然是一个“变量”，没办法通过“不可变绑定”修改变量的值，但是重新使用可变绑定（通过变量遮蔽做到）之后，就可以通过“可变绑定”修改变量的值（此时如果没有其他引用指向这个变量，对这个变量的修改就是完全合法的，是符合“内存安全”的）
* 类型推导：可以从变量声明的当前语句中获取信息进行推导，还能通过上下文信息进行推导。Rust只允许“局部变量/全局变量”实现类型推导，函数签名等场景下是不允许的
* 类型别名：用type关键字给同一类型起个别名（type alias），例如：` type Age = u32; `。类型别名用在泛型场景：` type Double<T> = (T , Vec<T>); `
* 静态变量：用static关键字声明静态变量。例如：` static GLOBAL: i32 = 0; `。static语句同样也是一个模式匹配，与let语句不同，static声明的变量的生命周期是整个程序，从启动到退出，它占用的内存空间不会在执行过程中回收（全局变量的内存不是分配在当前函数栈上，函数退出时并不会销毁全局变量占用的内存空间，程序退出才会回收）。这是Rust中唯一声明全局变量的方法
* 由于Rust非常注重内存安全，全局变量的使用有许多限制：
  * 全局变量必须在声明的时候马上初始化
  * 全局变量的初始化必须是编译期可确定的常量，不能包括执行期才能确定的表达式、语句和函数调用
  * 带有mut修饰的全局变量，在使用的时候必须使用unsafe关键字
  * 禁止在声明static变量的时候调用普通函数，或者利用语句块调用其他非const代码。调用const fn是允许的，因为const fn是编译期执行的
  * Rust不允许用户在main函数之前或者之后执行自己的代码。所以，比较复杂的static变量的初始化一般需要用lazy方式，在第一次使用的时候初始化。如果需要使用比较复杂的全局变量初始化，推荐使用lazy_static库
* 常量：使用const声明的是常量而不是变量，因此一定不允许使用mut关键字修饰这个绑定。例如：` const GLOBAL: i32 = 0 `。
* 常量的初始化表达式也一定要是一个编译期常量。它与static变量的最大区别在于：编译器不一定会给常量分配内存空间，很可能被内联优化。以const声明常量不具备类似let语句的模式匹配功能