pub mod my_error {
    use std::fmt::{Debug, Display};

    #[derive(Debug)]
    pub struct AppError;

    impl Display for AppError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "An Error Occurred, Please Try Again!") // user-facing output
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        print!("{}", 22);
    }
}
