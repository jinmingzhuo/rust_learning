static mut COUNTER: u32 = 3;
fn add_static(i: u32) {
    unsafe {
        COUNTER += i;
    }
}
fn main() {
    println!("Hello, world!");
    add_static(33);
    unsafe {
        println!("{}", COUNTER);
    }
}
