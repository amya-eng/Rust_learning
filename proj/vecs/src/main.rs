fn main() {
    //动态数组
    let _v:Vec<i32> = Vec::new();     // 不使用，加上_
    let v1 = vec![1,2,3];             // 可推导，不声明
    //更新
    let mut v2 = Vec::new();
    v2.push(5);                    // 可推导，不声明
    v2.push(10);
    v2.push(15);
    //读取
    let third:&i32 = &v1[2];
    println!("v1[2]: {} ", third);
    //or
    match v1.get(2) {                 // get方法返回一个Option<&T>，当偶尔越界是允许的时候，用这个方法
        Some(third) => println!("{}", third),
        None => println!("none"),
    }
    // // 会报错的情况：v3可变的同时又不可变，因为v3连续存储，添加元素后可能导致扩容，需要整体重新分配内存地址
    // let mut v3 = vec![1,2,3,4,5];
    // let first = &v3[3];
    // v3.push(6);
    // println!("the fourth element is: {}", first);
    //遍历
    for i in &mut v2 {
        *i += 5;
    }
    for i in &mut v2 {
        println!("{} ", i);
    }
    //结合枚举，存储不同类型的数据
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.3),
        SpreadsheetCell::Text(String::from("your text here")),
    ];         // 枚举变体没有被读取过会各产生warning
}
