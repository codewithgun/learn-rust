// 32 bits, either i32 or f32

union IntOrFloat {
    i: i32,
    f: f32,
}

pub fn union_test() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    // the i field may be uninitialized
    let value = unsafe { iof.i };
    println!("Value is {}", value);
}
