// further reading
// http://hermanradtke.com/2015/05/03/string-vs-str-in-rust-functions.html
// http://stackoverflow.com/a/29998582/105282
// https://github.com/hoodie/concatenation_benchmarks-rs
// use std::iter::FromIterator;



fn main() {
    assert_eq!(dna::RibonucleicAcid::new("CGA"),
               dna::RibonucleicAcid::new("CGA"));
    assert!(dna::RibonucleicAcid::new("CGA") != dna::RibonucleicAcid::new("AGC"));

}

mod dna {
    #[derive(Debug, PartialEq)]
    pub struct RibonucleicAcid<'a> {
        strand: &'a str,
    }

    impl<'a> RibonucleicAcid<'a> {
        pub fn new(strand: &str) -> RibonucleicAcid {
            RibonucleicAcid { strand: strand }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct DeoxyribonucleicAcid<'a> {
        strand: &'a str,
    }

    impl<'a> DeoxyribonucleicAcid<'a> {
        pub fn new(strand: &str) -> DeoxyribonucleicAcid {
            DeoxyribonucleicAcid { strand: strand }
        }
        pub fn to_rna(&self) -> RibonucleicAcid {
            let mut strung = String::new();
            for c in self.strand
                         .chars()
                         .map(|x| match x {
                             'G' => 'C',
                             'C' => 'G',
                             'T' => 'A',
                             'A' => 'U',
                             _ => ' ',
                         }) {
                strung.push(c);
            }

            RibonucleicAcid::new(strung.as_str())
        }
    }
}

// fn old_main() {
// println!("{:?}", foo_string());
// let bar = "foo";
//
// println!("{} {}", bar, bar.replace("foo", "bar"));
//
// let blah = "blah";
// let bleh = blah.chars()
// .map(|x| match x {
// 'a' => 'e',
//
// _ => x,
// })
// .collect::<Vec<_>>(); // or Vec::from_iter(bleh);
//
// println!("{:?}", &bleh[..]);
//
// let mut bloh = String::new();
//
// for c in bleh {
// bloh.push(c);
// }
//
// println!("{:?}", bloh);
//
// println!("{:?}", foo_to_bar("hahahha"));
// assert_eq!((), bleh);
// }
//
// fn foo_string() -> String {
// let bar = String::from("foo_string");
// bar
// }
//
// fn foo_to_bar(phrase: &str) -> String {
// let mut bar = String::new();
// for c in phrase.chars()
// .map(|x| match x {
// 'a' => 'e',
//
// _ => x,
// })
// .collect::<Vec<_>>() {
// bar.push(c);
// }
// bar.push_str(phrase);
// bar.push_str("bar");
//
// bar
// }
//
//
//
// fn misc() {
//     let foo = "foo"; // str / slice
//     let bar = "bar".to_string(); // String::from("bar") is okay too
//
//     // as_string(foo); // won't work because it's a referenced string slice &str
//     as_string(foo.to_string()); // but this will work because we've coerced it into a String
//     as_string(bar); // okay, because it's a String
//
//     as_string_slice(&foo); // okay, borrows the reference
//     // as_string_slice(bar); // won't work because we're expecting a referenced string slice
//
//
//     // let mut fizz = String::new();
//     // fizz.push_str("buzz");
//     // println!("{}", fizz);
//     //
//     // replace_with_buzz(fizz);
//     // println!("{}", fizz);
// }
//
// fn as_string(thing: String) {
//     println!("as_string: {}", thing);
// }
//
// fn as_string_slice(thing: &str) {
//     println!("as_string_slice: {}", thing);
// }
// fn replace_with_buzz(fizz: &str) {
//     &fizz = "buzz";
// }
// fn concatenate_string(this: String, that: String) {}
// fn concatenate_str(this: &str, that: &str) {
//     let mut foo = this.clone();
//     foo.extends(that.clone());
//     println!("{:?}", foo);
//
// }
//
