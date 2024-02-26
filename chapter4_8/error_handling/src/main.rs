use error_handling::my_error::AppError;

fn main() {
    println!("Hello, world!");
    let n: Option<i32> = None;
    let s = Some(33);
    let a: Result<&str, &str> = Ok("hh");
    let b: Result<&str, &str> = Err("hhh");
    n.or(s);
    a.or(b);
    match produce_error() {
        Err(e) => eprintln!("{}", e),
        _ => println!("sucess"),
    }
    println!("{:?}", produce_error());
}
fn produce_error() -> Result<(), AppError> {
    Err(AppError)
}
