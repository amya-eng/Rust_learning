fn main() {
    let s = String::from("Hello world");
    let s_ = first_w(&s);            // 这个&必须加
    println!("{}", s_);

    let s_ = first_w2(&s[..]);
    println!("{}", s_);

    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    for element in slice.iter() {
        println!("{}", element);
    }
}

fn first_w(s: &String) -> &str {    // s[..i]为str类型，而非String类型，同时&str前的&必须加上，否则unknown size,编译报错
    let bytes = s.as_bytes();     // 转化字符串为字符数组，误写为asbyte
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_w2(s: &str) -> &str {
    for (i, item) in s.chars().enumerate() {
        if item == ' ' {              // 这里的item又为什么不用&呢
            return &s[..i];
        }
    }
    &s[..]
}
