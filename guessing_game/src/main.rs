use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Start Guessing");
    println!("=============================================");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");
    
        let mut guess= String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // read_line에서 사용자 입력을 받을 때, enter를 누름으로써 입력되기 때문에 guess에는 개행문자가 추가된다.
        // 예를 들어, 사용자 입력이 5인 경우 guess에는 '5'가 아니라 '5\n'이 저장된다.
        // 이러한 개행문자를 제거하기 위해 trim() 메서드를 사용한다. 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // expect("Please type a number!"); 
    
        println!("You geussed: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too large"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }



}
