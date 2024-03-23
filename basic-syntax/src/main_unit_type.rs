use std::any;

fn main() {
    let my_number = number();
    println!("{}", any::type_name_of_val(&my_number));
    let unit_type = unit_type();
    println!("{}", any::type_name_of_val(&unit_type));
}

fn number() -> i32 {
    8
}

// ()는 empty tuple 또는 unit type 
// 반환값이 존재하지 않는 것을 의미
fn unit_type() -> () {
    ()
}

// 세미콜론(;)이 있으면 -> 문장 ex) 변수 선언, 함수 호출
// 세미콜론(;) 없으면 -> 표현식 ex) 반환값

// mismatched types , 해당 함수는 i32를 반환해야 하지만 unit type 반환하기 때문
// unit type은 java의 void와 비슷함
// 8; 은 표현식이 아닌 문장이므로 반환값이 존재하지 않게되므로 에러 발생
// fn number() -> i32 {
//     8;
// }