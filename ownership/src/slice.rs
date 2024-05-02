use core::slice;

fn main() {
    let mut s = String::from("hello world and yjk490");

    let x = first_word_by_slice(&s);
    
    // s.push_str("test");
    println!("{x}");    // hello 출력

}

fn first_word(s :&String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_by_slice(s :&String) -> &str { // &str은 문자열 슬라이스 타입
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]  // 공백이 없으면 전체 문자열 슬라이스 반환
}