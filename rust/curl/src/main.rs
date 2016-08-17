extern crate curl;

#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    FailedAuth,
    NoInternet,
    BadUrl,
}

fn get_web(url: &str) -> Result<String, Error> {
    use curl::easy::{Easy, List};

    let mut handle = Easy::new();
    let mut data = Vec::new();

    try!(handle.url(&url).map_err(|_| Error::BadUrl));
    handle.fail_on_error(true);
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
                    data.extend_from_slice(new_data);
                    Ok(new_data.len())
                })
                .unwrap();
        if transfer.perform().is_err() {
            return Err(Error::NoInternet);
        }
    }

    let data_string = String::from_utf8(data.clone());
    Ok(format!("get_web: {}", &data_string.unwrap()))
}

fn main() {
    println!("{:?}", get_web("http://localhost:32768/status/200"));
    println!("{:?}", get_web("http://localhost:32768/status/500"));
    println!("{:?}", get_web("http://localhost:32768/get"));
}

#[allow(dead_code)]
fn post_test() {
    use curl::easy::{Easy, List};
    let mut handle = Easy::new();

    let base_url = "http://localhost:32768/post";
    let base_params = "?browser=firefox&sensor=true";
    let mut url = String::new();
    url.push_str(base_url);
    url.push_str(base_params);

    // url.push_str(BASE_URL);
    // url.push_str(BASE_PARAMS);

    let mut url_params = String::new();
    url_params.push_str("&wifi=mac=:123|");

    let encoded_params = handle.url_encode(url_params.as_bytes());
    println!("{:?}", encoded_params);
    url.push_str(&encoded_params);

    let mut list = List::new();
    list.append("Accept: application/json").unwrap();

    handle.post(true).unwrap();
    handle.post_field_size(0).unwrap();

    handle.http_headers(list).unwrap();

    let mut data = Vec::new();
    handle.url(&url.to_string()).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|new_data| {
                    data.extend_from_slice(new_data);
                    Ok(new_data.len())
                })
                .unwrap();
        transfer.perform().unwrap();
    }

    let data_string = String::from_utf8(data.clone());
    println!("{}", data_string.unwrap());

}

#[allow(dead_code)]
fn works_but_only_stdout() {
    use curl::easy::{Easy, List};
    let base_url = "http://localhost:32768/post";
    let base_params = "?browser=firefox&sensor=true";

    let mut easy = Easy::new();
    let mut url = String::new();

    url.push_str(base_url);
    url.push_str(base_params);

    let mut url_params = String::new();
    url_params.push_str("&wifi=mac=:123|");

    let encoded_params = easy.url_encode(url_params.as_bytes());
    println!("{:?}", encoded_params);

    url.push_str(&encoded_params);

    let mut list = List::new();
    list.append("Accept: application/json").unwrap();

    easy.url(&url.to_string()).unwrap();

    easy.post(true).unwrap();
    easy.post_field_size(0).unwrap();

    easy.http_headers(list).unwrap();

    use std::io::{stdout, Write};
    easy.write_function(|data| Ok(stdout().write(data).unwrap()))
        .unwrap();

    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}
