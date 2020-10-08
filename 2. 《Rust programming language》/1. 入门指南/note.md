# 学习笔记

## 安装

* Linux or MacOS：` $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh `
* Windows：访问 ` https://www.rust-lang.org/tools/install/  `
* 手动添加Rust到系统环境变量：` $ source $HOME/.cargo/env `
* 添加环境变量到配置文件：` $ export PATH="$HOME/.cargo/bin:$PATH" `
* 为了Rust正常编译，通常还需要一个链接器，C语言编译器通常带有可用的链接器
* Windows下为了获得构建工具，建议先安装visual studio
* 升级Rust到最新版本：` $ rustup update `
* 卸载Rust：` $ rustup self uninstall `
* 查看版本：` $ rustc --version `
* 论坛：`  https://users.rust -lang.org/  `
* Stack Overflow：` http://stackoverflow.com/questions/tagged/rust/ `
* 打开本地文档：` $ rustup doc `
* 格式化代码：` $ rustfmt source_code.rs `

## Hello, World!

* Rust源文件以` .rs `结尾
* Rust源文件名、项目文件夹名如果由多个单词构成，用下划线连接起来，如：` hello_world.rs `
* Rust可执行程序的入口是main函数
* Rust采用的缩进风格是4个空格而不是tab
* 使用感叹号!结尾的通常是Rust的宏
* Rust语句以分号结尾
* Windows操作系统下，.pdb文件存放debugging信息
* 编译源代码：` $ rustc source_code.rs `

## Hello, Cargo!

* Cargo是Rust的构建系统和包管理器，能帮助组织你的项目代码
* 构建项目：` $ cargo new project_name `
* cargo new时，使用 --vcs 控制cargo是否启用支持版本控制系统功能，例如git
* TOML：Tom's Obvious, Minimal Language 格式。
* Rust中，代码包被描述为crates
* 构建cargo生成的项目：` $ cargo build `，可执行文件生成在target/debug目录下
* 用cargo运行项目：` $ cargo run `，如果没有build最新版本的源代码，则自动编译
* Cargo.lock文件：自动生成，无须手动更改，用于跟踪项目相关依赖的版本
* 快速检查代码、编译代码，但不生成可执行文件：` $ cargo check `，因为不生成可执行文件，所以比build速度快
* 项目准备正式发布则用：` $ cargo build --release `。可执行文件生成在target/release目录下，构建时间比build长，可执行文件运行效率更高