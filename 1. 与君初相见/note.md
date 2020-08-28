# 学习笔记

## 1.0 概述

* Rust是一门系统编程语言，它有三大特点：
  * 运行快
  * 防止段错误
  * 保证线程安全
* 系统编程是相对于应用级编程而言，意味着更底层的位置，更接近于硬件层次并为上层的应用软件提供支持，特点：
  * 可以在资源非常受限的环境下执行
  * 运行时开销很小，非常高效
  * 很小的运行库，甚至没有
  * 可以允许直接的内存操作

## 1.1 版本和发布策略

* 编译器源码：<https://github.com/rust-lang/rust>
* 语言设计和相关讨论：<https://github.com/rust-lang/rfcs>
* 标准库文档：<https://doc.rust-lang.org/std/>
* 编译器的版本号：主版本.次版本.修订号
  * 主版本：做了不兼容的API修改
  * 次版本：做了向下兼容的功能性新增
  * 修订号：做了向下兼容的问题修正
* Rust多渠道发布策略：
  * nightly版本：每天在主版本上自动创建出来的版本，功能最多、更新最快、存在问题的可能性更大。新功能首先在这个版本上开启，供用户试用。nightly版本中使用试验性功能，必须手动开启feature gate：在项目入口文件中加` #![feature(...name...)] `
  * beta版本：每隔一段时间，将一些在nightly版本中验证过的功能开放给用户使用，可以看做stable版本的“预发布”
  * stable版本：正式版本，每隔6星期发布一个新版本。stable版本是保证向前兼容的
* Rust语言每个相对复杂的新功能，都要经过RFC(Request For Comment)设计步骤。编写一份RFC，其中包括：功能的目的、详细设计方案、优缺点探讨等。RFC->Nightly->Beta->Stable
* Rust的edition策略：有时某些新功能确实需要一定程度上破坏兼容性，为了最大化地减少这些变动给用户带来的影响，让Rust的兼容性保证是一个有时限的长度而不是永久。

## 1.2 安装开发环境

* 使用rustup工具安装整套工具链：编译器、标准库、cargo
* Windows上，Rust支持两种ABI(Application Binary Interface)：
  * 一种是原生的MSVC版本：需下载Visual C++的工具链
  * 另一种是GNU版本：跟MinGW生成的库打交道
* 工具链：
  * rustc：编译器
  * cargo：包管理器，导入或发布开源库。官方管理仓库<https://crates.io/>
  * cargo-fmt、rustfmt：源代码格式化工具
  * rust-gdb、rust-lldb：调试器
  * rustdoc：文档生成器
  * rls、racer：为编辑器准备的代码提示工具
  * rustup：管理工具链下载、更新的工具
    * ` rustup self update `：更新rustup本身
    * ` rustup self uninstall `；卸载rust所有程序
    * ` rustup update `：更新工具链
    * ` rustup install nightly `：安装nightly版本的编译工具链
    * ` rustup default nightly `：设置默认工具链是nightly版本
* 中国科技大学代理服务：
  * 网址：<https://lug.ustc.edu.cn/wiki/mirrors/help/rust-static>
  * 建议国内用户设置好环境变量后再使用rustup：
    * ` export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static `
    * ` export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup `
  * 设置USTC的开源库代理服务，在$HOME/.cargo目录下创建名为config的文本文件，内容:

```text
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

* RLS(Rust Language Server)是官方提供的一个标准化的编辑器增强工具：
  * 地址：<https://github.com/rust-lang-nursery/rls>
  * 安装RLS：
    * rustup self update
    * rustup update nightly
    * rustup component add rls --toolchain nightly
    * rustup component add rust-analysis --toolchain nightly
    * rustup component add rust-src --toolchain nightly

## 1.3 Hello World

* 源码后缀名 .rs
* 行注释符号：//
* 块注释符号：/**/
* 默认情况下main函数是可执行程序的入口点。main无参数、无返回值
* 局部变量声明使用let关键字
* 每条语句使用分号结尾
* 语句块使用大括号
* 空格、换行、缩进不是语法规则的一部分
* println!，后面的感叹号代表这是一个宏。Rust中的宏与C/C++中的宏是完全不一样的东西。可以把它理解为一种安全版的编译期语法扩展。标准输出宏可以完成编译期格式检查，更加安全

## 1.4 Prelude

* Rust的代码从逻辑上是分crate和mod管理的：
  * crate：理解为项目，每个crate是一个完整的编译单元，它可以生成为一个lib或者exe可执行文件
  * mod：crate内部由mod管理，可以理解为namespace
  * 使用use语句把其他模块中的内容引入到当前模块中
* std：极简标准库，极少数嵌入式系统无法使用。Rust编译器对标准库有特殊处理，默认情况下编译器自动引入对标准库的依赖
* 标准库提供了std::prelude模块，在这个模块中导入了常见的type、trait、function、macro等。编译器会为用户写的每一个crate自动插入：` use std::prelude::*; `

## 1.5 Format格式详细说明

* Rust中的宏都是用同样的格式，详细参考：std::fmt模块说明

```rust
fn main() {
  println!("{}", 1);    // 默认用法
  println!("{:o}", 9);    // 八进制
  println!("{:x}", 255);    // 十六进制，小写
  println!("{:X}", 255);    // 十六进制， 大写
  println!("{:p}", &0);    // 指针
  println!("{:b}", 15);    // 二进制
  println!("{:e}", 10000f32);    // 科学计数（小写）
  println!("{:E}", 10000f32);    // 科学计数（大写）
  println!("{:?}", "test");    // 打印Debug
  println!("{:#?}", ("test1", "test2"));    // 带换行和缩进的Debug打印
  println!("{} {} {}", a="x", b="y");   // 命名参数
}
```