#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

/// Rules of References
/// 1 - At any time you can either both not both (in scope)
///     1 - One mutable reference
///     2 - Any number of immutable references
// 2 - References must always be valid (no danglers)
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> String {
    let s = String::from("hello"); // s goes into scope and has String value
    s // value moves to reference_to_nothing
} // s falls out of scope

// will not compile
// fn dangle() -> &String { // dangle returns a reference to a String
// let s = String::from("hello"); // s is a new String
// &s // we return a reference to the String, s
// }   // Here, s goes out of scope, and is dropped. It memory goes away.
// Danger!
//
fn cannot_have_mutable_ref_if_you_already_have_an_immutable() {
    let mut s = String::from("hello");
    // { you'd have to place the immutable references in a scope
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // } so they'd fall out of scope before the mutable reference

    // wrapping r3 in a scope won't fix E0502, because r1 and r2 would still be in scope
    // let r3 = &mut s; // BIG PROBLEM , remove comment to see E0502
}

fn one_mutable_reference() {
    let mut s = String::from("hello");

    {
        // removing this scope will trigger E0499, can't have more than one mutable reference to s at anytime
        let r1 = &mut s;
    }

    let r2 = &mut s;

}

fn mutable_borrowing() {
    let mut s: String = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    // to make a reference mutable
    // 1 - variable must be mutable: mut foo:<T>
    // 2 - refernce must be mutable: &mut bar
    // 3 - fn accepting refernce must be mutable: bar: &mut <T>
    some_string.push_str(", world");
}

fn calculate_length_by_referencence() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // pass reference to fn (borrows)

    println!("The length of '{}' is {}.'", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // s now has a reference to the value of s1
    let length = s.len();

    length
} // s goes out of scope, but nothing happens because it was a reference rather than moved
