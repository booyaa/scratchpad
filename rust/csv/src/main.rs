extern crate csv;

fn main() {
    let data = "
home, 51.031337, -0.403123
work, 51.521251, -0.203586
, 23.123, -5.2312
";

    let mut rdr = csv::Reader::from_string(data).has_headers(false);

    for row in rdr.decode() {
        let (x1, x2, x3): (Option<String>, f64, f64) = row.unwrap();
        println!("{} {} {}", x1.unwrap_or("".to_string()), x2, x3);
    }


}
