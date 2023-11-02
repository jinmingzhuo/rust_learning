enum MyEnum {
    A = 1,
    B,
    C,
}

fn main() {
    // 将枚举转换成整数，顺利通过
    let x = MyEnum::C as i32;
    println!("{}", x);
    // 将整数转换为枚举，失败
    // match x {
    //     MyEnum::A => {}
    //     MyEnum::B => {}
    //     MyEnum::C => {}
    //     _ => {}
    // }
}
