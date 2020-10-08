# 学习笔记

## 创建一个新的项目

* crate是Rust的最小编译单元，一个package（可以翻译为包）包含多个crate（可以翻译为单元包）

## 处理一次猜测

* 默认情况下，Rust会将prelude(预导入)模块内的条目自动引入到每一段程序中，其中包含了一小部分相当常见的类型
* fn：声明一个新的函数
* let：创建一个变量，Rust中变量默认不可变。加mut关键字使变量可修改
* 单行注释：//
* String::new()是String类型的关联函数（associated function），有些语言称为静态方法（static method），是针对类型本身而不是实例来定义的。
* String::new()返回String类型实例。String是标准库提供的，内部采用UTF-8编码并可按照需求扩展自己大小的字符串类型
* io::stdin()函数返回std::io::Stdin的实例，被用作句柄来处理终端中的标准输入
* 参数前面加&意味着当前参数是一个引用。与变量一样，引用（reference）也是默认不可变的，所以需要用` &mut guess `而不是` &guess `
* Rust标准库中，可找到许多以Result命名的类型，通常是各个自模块中Result泛型的特定版本。Result是一个枚举类型，由一系列固定的值组合成，这些值被称为枚举的变体。对于Result而言，有Ok、Err两个变体。
  * Ok：表明当前操作成功，并附带代码产生的值
  * Err：表明当前操作失败，并附带引发失败的具体原因
* 以read_line为例：expect是Result类型定义的方法
  * 假如io::Result实例的值是Err，那么expect方法就中断当前程序，并将传入的字符串参数显示出来
  * 假如io::Result实例的值是Ok，那么expect方法就会提取Ok中附带的值，并将它作为结果返回给用户

## 生成一个保密数字

* 在Cargo.toml中的[dependencies]下添加需要依赖的外部crate，然后执行：` $ cargo build `
* crates.io是人们用于分享各种开源Rust项目的网站。Cargo可以从注册表（registry）中获得所有可用库的最新版本信息，这些信息通常从crates.io上复制过来
* 当cargo build 报错 “Blocking waiting for file lock on package cache” 时，删除用户文件夹下.cargo/.package-cache目录以及项目文件下target目录，再重新执行cargo build
* cargo将计算依赖关系并写入Cargo.lock文件中，重新构建项目时，如果Cargo.lock文件存在，则根据其中的依赖版本确保构建是可重现的。再次构建项目时，Cargo会优先检查Cargo.lock，假如文件中存在已经指明具体版本的依赖库，那么它会跳过计算版本号的过程，并直接使用文件中指明的版本
* ` $ cargo update `：强制Cargo忽略Cargo.lock文件，升级依赖的版本，但仅限于小版本改动，例如：v0.3.14->v0.3.18，若要升级到v0.4.x版本，需要先变更Cargo.toml
* ` $ cargo doc --open `：项目所用依赖的API文档展示出来
* 包（crate）
  * 构建项目生成可执行程序的二进制包（binary crate）
  * 引用的包则是用于复用功能的库包（library crate）
* 随机数生成：rand::thread_rng返回一个特定的随机数生成器，它位于本地线程空间，并通过操作系统获得随机数种子。这个随机数生成器的gen_range方法，在rand::Rng这个trait中定义，要将这个trait引入作用域
* 你无法在使用第三方包时凭空知晓自己需要使用什么样的trait或什么样的函数，而是需要在各类包文档中找到相关的使用说明。` $ Cargo doc --open ` 可以为你在本地构建一份有关所有依赖的文档
* std::cmp::Ordering类型，与Result类型相同，也是枚举类型。它拥有Less、Greater、Equal这三个变体
* match表达式由数个分支组成，每个分支都包含一个用于匹配的模式（pattern），以及匹配成功后要执行的相应的代码。Rust中的match结构和模式非常强大，提供了依据不同条件执行不同代码的的能力
* Rust允许用同名的新变量来隐藏旧变量的值，这一特性通常被用于需要转换值类型的场景中
* break语句可以跳出loop无限循环