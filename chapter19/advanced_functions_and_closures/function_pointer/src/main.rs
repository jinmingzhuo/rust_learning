fn main() {
    println!("Hello, world!");
    println!("{}", do_twice1(add_one, 4));
    let list_of_numbers = vec![1, 2, 3];
    let a: Vec<_> = list_of_numbers.iter().map(|i| i + 100).collect();
    println!("{:?}", a);
    // let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    println!("{:?}", list_of_statuses);
}
#[derive(Debug)]
enum Status {
    Value(u32),
    Trunk(String),
    Stop,
}
fn add_one(i: i32) -> i32 {
    i + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(f(arg))
}
fn do_twice1<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(f(arg))
}
