fn main() {
    let my_name = "yjk490";
    let my_age = 28;
    
    println!("name -> {}, age -> {}", my_name, my_age);
    println!("name -> {my_name}, age -> {my_age}");
    println!(
        "name -> {0}, age -> {1}",
        my_name,
        my_age
    )
}
