use std::fs::File;
use std::io::Read;

fn main() {
    // let mut file = File::open("Cargo.toml").unwrap();

    use std::path::Path;
    assert_eq!(Path::new("/Users/booyaa/Dev/rust/foundry/wifiscanner/tests/fixtures/airport/airport01.txt")
                   .is_file(),
               true);

    let mut file =
        File::open("/Users/booyaa/Dev/rust/foundry/wifiscanner/tests/fixtures/airport/airport01.\
                    txt")
            .unwrap();



    let mut contents: Vec<u8> = Vec::new();
    // Returns amount of bytes read and append the result to the buffer
    let result = file.read_to_end(&mut contents).unwrap();
    println!("Read {} bytes", result);

    // To print the contents of the file

    let filestr = String::from_utf8(contents).unwrap();
    // println!("{:?}", filestr);

    let mut lines = filestr.lines();
    let first_line = lines.next().unwrap();

    println!("header: {}", first_line);

    for line in lines {
        println!("other: {}", line);
    }
}
