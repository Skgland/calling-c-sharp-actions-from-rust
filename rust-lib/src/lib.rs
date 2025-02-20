mod c_sharp;

pub fn normal_rust_fn<S: Fn(), F: Fn(i32), Fun: Fn() -> u8, O: Fn(i32, u8)>(
    success: S,
    failure: F,
    fun: Fun,
    other: O,
) {
    println!("Rust says hi!");

    let value = rand::random();
    if value % 2 == 0 {
        success();
    } else {
        failure(value);
    }

    other(value, fun());
}
