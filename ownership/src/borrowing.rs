fn main() {
    let str_1 = String::from("yjk");
    let length = calculate_length(&str_1); // 매개변수로 레퍼런스를 전달한다. 즉, 소유권을 빌려줬기 때문에 이후로도 해당 변수는 유효하다.

    println!("str_1 : {str_1}, length : {length}"); // 출력 -> str_1 : yjk, length : 3
}

fn calculate_length(str: &String) -> usize {
    str.len()
}