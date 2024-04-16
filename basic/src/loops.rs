fn main () {
    // 1. 반복문에서 값 반환하기
    let mut counter = 0;
    let result = loop {
        counter += 1;

        // 매 반복마다 counter가 10인지 검사, 참이면 반복 종료
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result : {result}\n");

    // 2. 루프 라벨로 여러 반복문 사이에서 모호함 없애기
    let mut count = 0;
    'counting_up : loop {
        println!("count : {count}");
        let mut remaining = 10;

        loop {
            println!("remaining : {remaining}");
            if remaining == 8 { 
                break;
            }
            if count == 2 { 
                break 'counting_up; // 현재 루프가 아닌 라벨 이름의 루프에서 탈출
            }

            remaining -= 1;
        };

        count += 1;
    };
    println!("End count : {count}");
}