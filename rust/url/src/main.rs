extern crate url;

use url::percent_encoding;

fn make_url(query: &str) -> String {
    let raw = &query.as_bytes();
    let encoded = percent_encoding::percent_encode(&raw, percent_encoding::QUERY_ENCODE_SET);
    format!("{}", encoded)
}
fn main() {

    println!("{}", make_url("!so [rust] Hello, world!"));
}
