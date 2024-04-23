fn main() {
    let str_1 = String::from("yjk");       
    let str_2 = gives_and_takes_ownership(str_1); // 1. str_1의 소유권이 이 함수의 매개변수인 name으로 이동한다. 이 라인 이후로 str_1은 무효화된다.
                                                                // 5. gives_and_takes_ownership() 함수로부터 반환된 result의 소유권이 str_2로 이동했다.

    println!("{str_2}");                                        // 6. 변수 str_2는 소유권을 갖고 있으므로 유효한 변수다.
}

fn gives_and_takes_ownership(name: String) -> String {  // 2. 이 함수의 스코프를 벗어나면 name의 소유권이 해제된다.
    let result = format!("{name}, Hello");      // 3. 변수 result를 선언함으로써 소유권이 발생하고 이 스코프 내에서 유효하다.

    result                                              // 4. 변수 result를 반환함으로써 이 함수의 호출자쪽으로 result의 소유권이 이동한다.
} 