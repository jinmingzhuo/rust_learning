// 不安全的实现，涉及直接的指针操作
fn unsafe_function(data: *mut i32) {
    // 这里可能包含不安全的内存操作
    unsafe {
        *data = 42;
    }
}

// 安全的抽象层，封装了不安全的实现
fn safe_function(data: &mut i32) {
    // 这里是安全的代码，调用不安全的实现
    unsafe_function(data as *mut i32);
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len, "out of range!");

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
extern "C" {
    fn abs(input: i32) -> i32;
}
fn main() {
    let mut value = 0;

    // 使用安全的抽象层，而不直接调用不安全的函数
    safe_function(&mut value);

    println!("Value after safe operation: {}", value);

    let mut num = 6;
    let r1 = &num as *const i32;
    let mut r2 = &mut num as *mut i32;
    println!("r1={:?}", r1);
    println!("r2={:?}", r2);

    let mut j = 33;
    let j1 = &j as *const i32;
    let j2 = &mut j as *mut i32;
    unsafe {
        // r1 = j1;
        r2 = j2.add(10);
        println!("r1 is:{}", *r1);
        println!("r2 is {}", *r2);
    }

    let mut v = vec![1, 1, 1, 2, 2, 2];
    let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(&mut v, 3);
    println!("{:?}", a);
    println!("{:?}", b);

    unsafe {
        println!("test abs fn: {}", abs(-3));
    }
}
