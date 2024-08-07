fn main() {
    let s = String::from("Hello world!");

    let slice = &s[0..2];
    println!("{}", slice);
    let slice = &s[..2];
    println!("{}", slice);
    let slice = &s[6..];
    println!("{}", slice);
    let slice = &s[..];
    println!("{}", slice);
}
