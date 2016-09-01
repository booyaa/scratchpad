fn main() {
    let mut input_string = String::new();

   std::io::stdin().read_line(&mut input_string).unwrap();

    println!("Hello, World.\n{}",input_string.trim());    
}
