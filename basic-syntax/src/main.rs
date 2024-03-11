use std::any;
fn main() {

    // Integer
    let default_int = 50;   // 정수형의 기본 타입으로 i32 지정
    println!("type of default_int = {}\n", any::type_name_of_val(&default_int));
    
    let first: u8 = 10;                 // u8 타입 지정
    let second = 20;                // 타입 미지정
    let result = first + second;    // 타입 추론
    println!("first + second = {}\n", result);
    
    let my_int1 = 9u8;              // 숫자의 경우 타입을 값 바로 왼쪽에 적어도 됨
    let my_int2 = -1_i8;            // 언더바(_)도 허용
    let my_int3= 1_000_000_u32;     // 타입뿐만 아니라 숫자 사이에도 언더바 적용 가능
    
    println!("my_int1 = {}",my_int1);
    println!("my_int2 = {}",my_int2);
    println!("my_int2 = {}\n",my_int3);
    
    // float
    let default_float = 8.0;    // 실수형의 기본 타입은 f64
    println!("type of default_float = {}\n", any::type_name_of_val(&default_float));

    // Char
    let letter = 'A';
    let space = ' ';
    let cat_face = '😺';

    println!("letter = {}", letter);
    println!("space = {}", space);
    println!("cat_face = {}\n", cat_face);


    // Type casting
    let my_char = 'A';
    println!("'A' cast to Integer :  {}\n", my_char as i8);   // 65, 유니코드 문자를 정수형으로 변환

    let my_int: i8 = 30;
    let casted_my_int = my_int as u32;
    println!("my_int's type: {}", any::type_name_of_val(&casted_my_int));      // u32
}