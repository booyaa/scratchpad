use std::env;

fn main() {
    let mut args = env::args();

    let _program_name = args.next().unwrap();
    let csv = args.next().expect("missing csv file");

    println!("app: {} csv: {}", _program_name, csv);
}
