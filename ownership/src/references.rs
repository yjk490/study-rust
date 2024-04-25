fn main() {
    let str = String::from("hello");
    add_string(&str);

    let mut name = String::from("YJK");
    add_string_by_mutref(&mut name);
    println!("name : {name}"); // name : YJK490 출력

    let a = &mut name;
    let b = &mut name;
    // println!("x : {a}, y : {b}");
}

fn add_string(target: &String) {
    // target.push_str(", world"); // 컴파일 에러 발생
}

fn add_string_by_mutref(target: &mut String) {
    target.push_str("490");
}