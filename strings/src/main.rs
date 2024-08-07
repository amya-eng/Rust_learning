fn main() {
    //创建
    let mut s1 = String::new();

    let data = "hello";
    let mut s2 = data.to_string();

    let s3 = "world".to_string();

    let s4 = String::from("hello, world");
    // 更新
    s2.push_str(" Rust ");
    s2.push('!');
    // 拼接

    let s5 = String::from("hello ");
    let s6 = String::from(" world ");
    let s7 = s5 + &s6; // + 调用add方法，同时&String类型作为参数被强制转换为&str
    // s5所有权被转移
    let s8 = String::from("hi");
    let s = format!("{}-{}", s6, s8);

    //字符串索引
    let l = s8.len();
    println!("len : {}", l);      // 打印：len : 2
    let slice = &s[1..5];
}
