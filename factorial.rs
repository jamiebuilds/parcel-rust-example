#[no_mangle]
pub fn factorial(num: i32) -> i32 {
    if num == 0 || num == 1 {
        return 1;
    } else {
        return num * factorial(num - 1);
    }
}
