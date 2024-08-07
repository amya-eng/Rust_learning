use std::fs::File;
fn main() {
    // 报错
    // panic!("crash and burn");

    // 报错 -回溯
    // let v = vec![1,2,3];
    // v[99];

    // 报错 -result的返回值, 故意带上错误的类型标记，让编译器告诉我们File::open的返回类型
    let f: u32= File::open("hello.txt");
    
}
