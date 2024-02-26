fn main() {
    println!("Hello, world!");
    let a = &32 as *const i32;
    let a1 = &32 as *const i32;
    let a2 = &32 as *const i32;
    let h = "jmz" as *const str;
    let h1 = "jmz" as *const str;
    println!("{:?}", a);
    println!("{:?}", a1);
    println!("{:?}", a2);
    println!("{:?}", h);
    println!("{:?}", h1);
}
