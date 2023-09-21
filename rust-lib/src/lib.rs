mod c_sharp;

pub fn normal_rust_fn<S: Fn(), F: Fn(i32)>(success: S, failure: F) {
    let value = rand::random();
    if value % 2 == 0 {
        success();
    } else {
        failure(value);
    }
}
