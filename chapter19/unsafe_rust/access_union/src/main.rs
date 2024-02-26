// 使用 struct 定义一个 Point 结构体
struct Point {
    x: i32,
    y: i32,
}

// 使用 union 定义一个 Color 联合体
union Color {
    rgba: u32,
    components: [u8; 4],
}

fn main() {
    // 使用 struct 创建一个 Point 实例
    let point = Point { x: 10, y: 20 };

    // 使用 union 创建一个 Color 实例
    let color = Color {
        rgba: 0xFF_00_00_FF,
    };

    // 访问 struct 中的字段
    println!("Point: ({}, {})", point.x, point.y);

    // 访问 union 中的字段（需要使用 unsafe 块）
    unsafe {
        println!("Color: RGBA = 0x{:X}", color.rgba);
        println!("Color: RGBA = 0x{:X}", color.components[2]);
    }
}
