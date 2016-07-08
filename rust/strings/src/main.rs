// further reading
// http://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html
// http://stackoverflow.com/a/29998582/105282
fn main() {
    let foo = "foo";
    let bar = "bar".to_string(); // String::from("bar") is okay too

    // as_string(foo); // won't work because it's a referenced string slice &str
    as_string(foo.to_string()); // but this will work because we've coerced it into a String
    as_string(bar); // okay, because it's a String

    as_string_slice(foo); // okay
    // as_string_slice(bar); // won't work because we're expecting a referenced string slice
}

fn as_string(thing: String) {
    println!("as_string: {}", thing);
}

fn as_string_slice(thing: &str) {
    println!("as_string_slice: {}", thing);
}

fn concatenate_string(this: String, that: String) {}

fn concatenate_str(this: &str, that: &str) {}
