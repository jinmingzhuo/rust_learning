use std::convert::TryFrom;

enum MyEnum {
    A = 1,
    B,
    C,
}

impl TryFrom<i32> for MyEnum {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == MyEnum::A as i32 => Ok(MyEnum::A),
            x if x == MyEnum::B as i32 => Ok(MyEnum::B),
            x if x == MyEnum::C as i32 => Ok(MyEnum::C),
            _ => Err(()),
        }
    }
}

use std::convert::TryInto;

fn main() {
    let x = MyEnum::C as i32;

    match x.try_into() {
        Ok(MyEnum::A) => println!("a"),
        Ok(MyEnum::B) => println!("b"),
        Ok(MyEnum::C) => println!("c"),
        Err(_) => eprintln!("unknown number"),
    }

    let a = 103;
    match a {
        x if x > 10 => println!(">10"),
        x if x == 10 => println!("10"),
        _ => println!("<10"),
    }
}
