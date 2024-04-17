fn main () {
    let x = 3;
    let y = 4;

    // Rust에서 if는 표현식이기 때문에 변수 값을 할당하는 데 사용할 수 있음
    let first_flag = if (x > y) { true } else { false };

    // if/else 블록이 반환하는 값이 서로다르면 컴파일 에러 발생
    // let second_flag = if (x > y) { x } else { "y is small than x"};
}