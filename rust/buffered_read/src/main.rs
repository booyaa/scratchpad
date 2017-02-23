extern crate regex;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;
use regex::Regex;

fn foo() -> io::Result<()> {
    let mut f = try!(File::open("dontcommitmebro/pg345.txt"));
    let mut buffer = vec![0; 30]; // 200];

    try!(f.seek(SeekFrom::Start(4105))); // chapter 1

    // read up to 10 bytes
    try!(f.read(&mut buffer));

    // println!("The bytes: {:?}", buffer);
    let bar = String::from_utf8(buffer).unwrap();
    splitter(&bar);
    Ok(())
}

fn splitter(data: &str) -> io::Result<()> {
    let words_re = Regex::new(r"\w+").unwrap();
    // let caps = words_re.captures(&data);
    let caps = words_re.replace_all(&data, "one");
    println!("{}\n-----------\n{}", &data, caps);
    Ok(())
}

fn main() {
    let _ = foo();
}
