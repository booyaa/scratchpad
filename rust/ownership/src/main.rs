#![allow(dead_code)]
#![allow(unused_variables)]


// remember String struct, but for now think of it as a tuple (ptr,len,capacity)
// ptr points the memory that holds the contents of the string

// rustc -W help
fn main() {
    let s1 = String::from("hello"); // s1 in scope
    let (s2, len) = calculate_length(s1);   // 1 - s1 moves to fn
                                            // 6 - s2 and len have ownership and go into scope

    println!("The length of '{}' is {}.'", s2, len); // 7 println now owns s2 and len
} // 8 s2, len go out of scope and are dropped. s1 moved and falls out of scope

fn calculate_length(s: String) -> (String, usize) {
    // 2 - s now owns s1's value
    let length = s.len(); // 3 - length in scope

    (s, length) // 4 - moves ownership of values s and length
} // 5 - s and length go out of scope

fn give_back() {
    let s1 = gives_ownership(); // 1-s1 goes into scope and takes ownership of "Hello" from some_string
    let s2 = String::from("hello"); // 5-s2 goes into scope with "hello"
    let s3 = takes_and_gives_back(s2); // 6-s2 is passed to fn
} // 9 s3 falls out of scope and is dropped. s2's moved and data already droped and falls out of scope. s1 falls out of scope and is dropped.

fn gives_ownership() -> String {
    let some_string = String::from("hello"); // 2-goes into scope
    some_string // 3-moves to s1
} // 4-goes out of scope? and dropped?

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 7 passes s2 back
} // 8 goes out of scope? and dropped


fn pass_values_to_funcs() {
    let s = String::from("Hello");  // s goes into scope
    take_ownership(s);              // s moves into function..
                                    // ... no longer valid
    let x = 5;                      // x goes into scope
    makes_copy(x);                  // but i32 is Copy, so it's okay to still
                                    // use x afterwards
} // x goes out of scope, then s. Since s was moved, nothing special happens
fn take_ownership(some_string: String) {
    // in scope
    println!("{}", some_string);            //
}

fn makes_copy(some_integer: i32) {
    print!("{}", some_integer);
}

fn copy_types() {
    // Primatives, tuples but not if they don't include primatives
    let x = 5;
    let y = x;
    println!("{}", x);

}
fn cloned_value() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy
    println!("{}", s1);
}

fn moved_value() {
    let s1 = String::from("hello");
    let s2 = s1; // shallow copy
    // uncomment to trigger use of moved value println!("{}", s1);
}
fn string_types() {
    let string_literal = "foo"; // &str, will be hardcoded into binary
    let string_type = String::from("bar"); //std::string:String, mem allocated at run time
    let mut s = String::from("Hello");
    s.push_str(", world!");
} // s frees memory after falling out of scope

fn variable_binding_scope() {
    // begin scope, a is not valid
    let a = "hello";                // a is valid from this point forward

}                                   // end of scope and a too
