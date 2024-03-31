#![allow(unused_doc_comments)]

fn main() {
    /**
     * 불변성
     */
    let x = 5;
    println!("The value of x is: {x}");
    
	// 아래 코드를 주석 해제하면 에러 발생
    // x = 10;  // Error, x에는 이미 값이 할당되었기 때문에 다른 값을 할당할 수 없음
    
    let mut y = 10;
    println!("The value of y is: {y}");
    
    y = 20;
    println!("The value of y is: {y}");
    
    /**
     * 섀도잉
     */
    
    let z = 5;
    
    {
        let z = z * 2;
        println!("Inner z: {z}");
    }
    
    println!("Outer z: {z}");
    
    let z = z + 2;
    println!("Shadowed z: {z}");

    // mut 키워드와의 차이점
    let space = "   ";
    let space = space.len();

    println!("space : {space}");

    let mut mut_space = "   ";
    // mut_space = mut_space.len();  // 이 라인을 주석 해제하면 에러 발생, 변수 mut_space는 문자열 타입이기 때문.
}
