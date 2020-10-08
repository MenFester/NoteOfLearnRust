struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user's name is: {}", user1.username);

    let user2 = build_user(String::from("13500177552@qq.com"), String::from("linguanqiang"));
    println!("username-2={}, email-2={}", user2.username, user2.email);
    println!("{} {}", user2.active, user2.sign_in_count);

    let user3 = build_user2(String::from("13500177552@qq.com"), String::from("linguanqiang"));
    println!("username-3={}, email-3={}", user3.username, user3.email);
    println!("{} {}", user3.active, user3.sign_in_count);

    let user4 = User {
        active: false,
        sign_in_count: 3,
        ..user3    // 不需要逗号结尾，看做展开原结构体实例中已经有了逗号
    };
    println!("username-4={}, email-4={}", user4.username, user4.email);
    println!("{} {}", user4.active, user4.sign_in_count);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.0);
    println!("{}", origin.0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}