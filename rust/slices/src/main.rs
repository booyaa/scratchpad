fn main() {
    println!("{}", first_word(&String::from("Hello World!")));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == 32 {
            return i;
        }
    }

    s.len()
}
