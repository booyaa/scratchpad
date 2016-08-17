use std::env;


fn pattern_match(key: &str) {
    println!("pattern matching");
    match env::var(key) {
        Ok(val) => println!("\tkey: {} Value: {}", key, val),
        Err(e) => println!("\tCouldn't find: {} - {}", key, e),
    };
}

fn if_let(key: &str) {
    println!("if let");
    if let Ok(val) = env::var(key) {
        println!("\tkey: {} Value: {}", key, val);
    } else {
        println!("\tfailed to find {}", key);
    }
}
fn main() {
    let key = "HOME";
    pattern_match(&key);
    pattern_match("no match");

    if_let(&key);
    if_let("no match");

    // how to return early
    let val_result = env::var(key);
    if val_result.is_err() {
        println!("Failed to find env {}", key);
        return;
    }

    println!("key: {} value: {}", key, val_result.unwrap());


    let value = env::var(key).expect("This won't trigger!");
    println!("expect\n\tkey: {} value: {}", key, value);

    // not a particular friendly error: thread '<main>' panicked at 'Couldn't find key!: NotPresent', ../src/libcore/result.rs:785
    println!("this will never trigger: {}",
             env::var("no match").expect("Couldn't find key!"));
}
