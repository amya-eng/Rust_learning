use std::collections::HashMap;
fn main() {
    //创建
    let mut scores = HashMap::new();
    let s = String::from("Blue");
    scores.insert(s, 10); // s的所有权被转移，之后不可用
    scores.insert(String::from("Red"), 5);
    //or
    let teams = vec![String::from("Blue"), String::from("Red")];
    let init = Vec![10, 50];
    let score: HashMap<_, _> = teams.iter().zip(init.iter()).collect(); // 其中类型标记不能省略

    //访问
    let team_name = String::from("Blue");
    let team_score = scores.get(&team_name);   // get返回的是Option<&V>
    // OR
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    // 更新
    // 覆盖旧值
    let mut s1 = HashMap::new();
    s1.insert(String::from("Blue"), 1);
    s1.insert(String::from("Blue"), 10);
    println!("{:?}", s1);
    // 仅在键没有对应值时插入数据
    s1.entry(String::from("red")).or_insert(5);
    println!("{:?}", s1);
    // 基于旧值更新新值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);  // count 指向word对应的值
        *count += 1; 
    }
    println!("{:?}", map);

}
