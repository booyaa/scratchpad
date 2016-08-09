extern crate test_fixtures;

#[test]
fn should_return_hotspot_name() {
    assert_eq!("hotspot name", test_fixtures::parse("asdasd"));
}

#[test]
fn parse_file_should_return_hotspot_name() {

    use std::path::PathBuf;
    // let mut path = PathBuf::from(".");
    let mut path = PathBuf::new();
    path.push("tests");
    path.push("fixtures");
    path.push("foo");
    path.push("foo.txt");

    let file_path = path.as_os_str();
    println!("file_path: {:?}", file_path);
    // let mut file_path = "tests/fixtures/foo/foo.txt";

    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(&file_path).unwrap();
    // let mut file = File::open("./tests/fixtures/foo/foo.txt").unwrap();

    let mut filestr = String::new();
    let result = file.read_to_string(&mut filestr).unwrap();
    println!("Read {} bytes", result);

    assert_eq!("hotspot name", test_fixtures::parse(&filestr));
}
