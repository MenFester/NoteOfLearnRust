fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500]; // 如果所有元素初始化为同样数据时的语法
    for item in &xs {
        println!("{}", item);
    }
    for item in ys.iter() {    // 用&ys报错
        println!("{}", item);
    }
}
