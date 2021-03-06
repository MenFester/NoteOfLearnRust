# 学习笔记

## 定义并实例化结构体

* 结构或结构体，是一种自定义数据类型，它允许我们命名多个相关的值并将它们组成一个有机的结合体。可以将结构体视作对象中的数据属性
* 结构体与枚举类型是用来创建新类型的基本工具，这些特定领域中的新类型同样可以享受到Rust编译时类型检查系统的所有优势
* 和元组一样，结构体中的数据可以拥有不同的类型。和元组不一样的是，结构体需要给每个数据赋予名字以便清楚地表明它们的意义，不再需要依赖顺序索引来指定或访问实例中的值
* 关键字struct被用来定义并命名结构体，并在随后的花括号中声明所有数据的名字及类型，这些数据也被称作字段
* 为了使用好结构体，我们需要为每个字段赋予具体的值来创建结构体实例。可以通过声明结构体名称，并使用一对花括号包含键值对来创建实例。赋值顺序不需要严格对应结构体中声明的顺序
* 可以用点号来访问实例中的特定字段，假如结构体实例是可变的，还可以通过点号来修改字段的值
* 一旦实例可变，那么实例中的所有字段都将是可变的，Rust不允许单独声明某一部分字段的可变性
* 可以在函数体最后一个表达式中构造结构体实例来隐式将实例作为结果返回
* 在函数中使用与结构体字段名相同的参数名可以让代码更加易于阅读
* 在变量名誉字段名相同时，使用简化版的字段初始化方法
* 使用结构体更新语法，可根据其他实例来创建新实例：双点号..表明剩下的还未被显示赋值的字段都与给定实例拥有相同的值
* 使用不需要堆字段命名的元组结构体来创建不同的类型：使用一种类似于元组的方式定义结构体，称为元组结构体。元组结构体同样拥有用于表明自身含义的名称，但无须在声明它时对其字段进行命名，仅保留字段的类型即可。一般来说，当你想要给元组赋予名字，并使其区别于其他拥有同样定义的元组时，你就可以使用元组结构体
* 定义元组结构体时依然使用struct关键字，并由结构体名称及元组中的类型定义组成
* 元组结构体的行为就像元组一样：可以通过模式匹配将它们解构为单独的部分，也可以通过点号及索引来访问特定字段
* Rust允许创建没有任何字段的结构体。这种结构体与空元组()十分相似，所以它们也被称为空结构体。当你想要在某些类型上实现一个trait，却不需要在该类型中存储任何数据时，空结构体就可以发挥相应的作用
* 我们希望结构体的实例拥有自身全部数据的所有权，这种情形下，只要结构体是有效的，那它携带的全部数据也就是有效的。我们也可以在结构体中存储指向其他数据的引用，不过这需要用到Rust中独有的生命周期功能。生命周期保证了结构体实例中引用数据的有效期不短于实例本身

## 一个使用结构体的示例程序

* println!宏可以执行多种不同的文本格式化命令，而作为默认选项，格式化文本中的花括号"{}"会告知println!使用名为Display的格式化方法：这类输出可以被展示直接的终端用户。所有基础类型都默认地实现了Display
* Rust没有为结构体提供默认的Display
* "{:?}"告知println!需要使用名为Debug的格式化输出，Debug是另外一种格式化trait
* 在结构体定以前添加` #[derive(Debug)] `注解，来派生Debug trait
* Rust提供了许多可以通过derive注解来派生的trait，它们可以为自定义的类型增加许多有用的功能

## 方法

* 方法与函数十分相似：它们都使用fn关键字及一个名字来进行声明；它们都可以拥有参数和返回值；另外，它们都包含了一段在调用时执行的代码
* 方法总是被定义在某个结构体（或者枚举类型、trait对象）的上下文中，并且它们的第一个参数永远都是self，用于指代该方法的结构体实例
* 为了在上下文中定义方法，需要将函数移动到一个由impl（implementation）关键字、后跟结构体名起始的代码块中，并把签名中的第一个参数和函数中使用该参数的地方改写为self
* 方法调用是通过实例后面加点号，并跟上方法名、括号及可能的参数来实现
* 如果想要在调用方法时改变实例的某些数据，那么就需要将第一个参数改写为` &mut self `
* 将第一个参数标记为self并在调用过程中取得实例的所有权方法并不常见，这种技术有可能会被用于那些需要将self转换为其他类型，且在转换后想要阻止调用者访问原始实例的场景
* 使用方法替代函数，能够避免在每个方法的签名中重复编写self的类型
* 当使用object.something()调用方法时，Rust会自动为调用者object添加` & 、&mut 或 * `，以使其能够符合方法的签名。因为方法有一个明确的作用对象：self的类型。Rust可以准确地推导出方法是否是只读的、需要修改的、获取数据所有权的
* 方法可以在self参数后增加签名来接收多个参数，就如同函数一样
* impl块还允许我们定义不用接收self作为参数的函数，它们被称为关联函数。关联函数常常被用作构造器来返回一个结构体的实例。我们可以在类型名称后添加` :: `来调用关联函数
* 每个结构体可以拥有多个impl模块