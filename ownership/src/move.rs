fn main() {
    let x = String::from("hello");
    let y = x;

    println!("x -> {x}");   // 컴파일 에러 발생, 변수 x에 할당된 값의 소유권이 y로 이동했기 때문
    println!("y -> {y}");
}