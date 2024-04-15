use std::result;

fn main() {
    // 함수의 매개변수와 반환 타입
    let result_1 = divide_three(13);
    let result_2 = plus_one(5);
}

fn divide_three(x: i32) -> f64 {
    x as f64 / 3.0
}

fn plus_one(x: i32) -> i32 {
    let result = x + 1;
    return result;
}