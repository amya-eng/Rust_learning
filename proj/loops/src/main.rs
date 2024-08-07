fn main() {
    let a = [1,2,3,4,5];
    oneway(a);
    twoway();
    fn oneway(a: [u32; 5]) {
        for element in a.iter() {
            println!("the value is : {}", element);
        }
    }
    fn twoway() {
        for num in (1..4).rev() {
            println!("{}!", num);
        }
        println!("LIFTOFF!");
    }
}
