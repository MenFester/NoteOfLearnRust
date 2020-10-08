fn print_slice(arr: &[i32]) {
    println!("Length: {}", arr.len());

    for item in arr {
        print!("{} \t", item);
    }
    println!("");
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500]; // 如果所有元素初始化为同样数据时的语法
    for item in &xs {
        println!("{}", item);
    }
    for item in ys.iter() {    // 用&ys报错
        println!("{}", item);
    }

    let v: [[i32; 2]; 3] = [[0, 0], [0, 0], [0, 0]];
    for i in &v {
        println!("{:?}", i);
    }

    fn mut_array(a: &mut [i32]) {
        a[2] =  5;
    }
    println!("size of usize : {:?}", std::mem::size_of::<usize>());    // 8
    println!("size of &[i32; 3] : {:?}", std::mem::size_of::<&[i32; 3]>());    // 8, 占用空间大小与指针相同
    println!("size of &[i32] : {:?}", std::mem::size_of::<&[i32]>());    // 16,  占用空间大小等于两个指针大小

    let mut mut_v: [i32; 3] = [1, 2, 3];
    {
        let s: &mut [i32; 3] = &mut mut_v;
        mut_array(s);
    }
    println!("{:?}", mut_v);

    let r1 = 1..10;
    for i in r1 {
        print!("{:?} \t", i);
    }

    println!("");

    use std::ops::Range;
    let r2 = Range {start: 1, end: 10};
    for i in r2 {
        print!("{:?} \t", i);
    }

    println!("");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let test: &[i32] = &arr;    // 局部变量是DST，编译正常。如果使用 let test: [i32]就报错
    print_slice(test);    // 函数参数是DST，编译正常
    println!("test: {:?}", test);
    print_slice(&arr[..]);
    print_slice(&arr[2..]);
    print_slice(&arr[..2]);

    let first = arr.get(0);
    let tenth = arr.get(10);
    println!("{:?}  {:?}", first, tenth);

    let v3 = &[10i32, 20, 30, 40, 50];
    for (index, value) in v3.iter().enumerate() {
        println!("{} {}", index, value);
    }
    let item = v3.iter().filter(|&x| *x % 2 == 0).nth(2);
    println!("{:?}", item);
}
