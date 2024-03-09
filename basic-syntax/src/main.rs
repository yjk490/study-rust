fn main() {

    // Integer
    let default_int = 50;   // 정수의 기본 타입으로 i32 지정

    let first: u8 = 10;     // u8 타입 지정
    let second = 20;    // 타입 미지정
    let result = first + second;    // 타입 추론
    println!("first + second = {}\n", result);

    // Char
    let first_letter = 'A';
    let space = ' ';
    let cat_face = '😺';


    // Type casting
    let my_char = 'A';
    println!("'A' cast to Integer :  {}", my_char as i8);   // 65, 유니코드 문자를 정수형으로 변환

    let my_int: i8 = 30;
    let casted_my_int = my_int as u32;
    println!("my_int's type: {}", std::any::type_name_of_val(&casted_my_int));      // u32
}