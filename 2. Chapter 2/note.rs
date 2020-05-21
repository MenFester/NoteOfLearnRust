fn test(condition: bool) {
    let x: i32;
    if condition {
        x = 1; // 初始化，不需要x是mut的
        println!("{}", x);
    }
}

fn main() {
    test(true);

    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let v = v; // 用变量遮蔽修改变量绑定的可变性
    for i in &v {
        println!("{}", i);
    }

    let player_scores = [("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19)];
    let players: Vec<_> = player_scores // Vec动态数组包含类型不清楚，在尖括号中用下划线代替
        .iter()
        .map(|&(player, _score)| player)
        .collect();
    println!("{:?}", players);

    let x;
    let y = 1_i32;
    x = 2_i32;
    println!("{} {}", x, y);
    static G1: i32 = 3;
    println!("{}", G1);
    static mut G2: i32 = 4;
    unsafe {
        // 可变全局变量无论读写都必须使用unsafe修饰
        G2 = 5;
        println!("{}", G2);
    }
}
