fn main() {
    // defination
    #[derive(Debug)]             // 为了打印，必须写在定义的上一行
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // 实例化
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("name"),
        active: true,
        sign_in_count:1,
    };

    user1.sign_in_count = 2;

    // 特殊的更新语法
    let user2 = User {
        email: String::from("zzs@qq.com"),     // 这里不能写成逗号
        username: String::from("123"),         
        ..user1
    };

    //打印结构体
    println!("usr2 is {:#?}", user2);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0, 0, 0);
    // 上black和origin是不同的类型，不可相互赋值



}
