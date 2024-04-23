fn main() {
    {   // 변수 x는 이 블럭 내에서만 유효
        let x = String::from("Hello");
    }   // 변수 x가 이 블럭을 벗어나면 메모리 해제, 내부적으로 drop 메서드 호출
    
    // x.str_push("world"); // 컴파일 에러 발생!
}
    
