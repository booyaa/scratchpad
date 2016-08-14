extern crate regex;
use regex::Regex;
// use regex::Error;

#[derive(Debug)]
enum Error {
    NoMatch,
    FailedToParse,
}

fn calc_signal(value: &i32) -> i32 {
    ((100 * value) / 100) / 2 - 100
}

fn get_signal(line: &str) -> Result<i32, Error> {
    // text: Signal level=58/100
    // want 58
    let mut parts = try!(line.split("=").nth(1).ok_or(Error::NoMatch)).split("/");
    let value = try!(try!(parts.nth(0).ok_or(Error::NoMatch))
                         .parse::<i32>()
                         .map_err(|_| Error::FailedToParse));

    Ok(calc_signal(&value))
}

fn get_signal_regex(line: &str) -> Result<i32, Error> {
    let re = Regex::new(r"^Signal level=(\d+)/100$").unwrap();
    let value_raw = re.captures(line).unwrap().at(1).unwrap(); // 0 returns the whole string?
    let value = try!(value_raw.parse::<i32>().map_err(|_| Error::FailedToParse));
    Ok(calc_signal(&value))
}

fn get_ssid_regex(line: &str) -> Result<String, Error> {
    let re = Regex::new(r#"^ESSID:"(.*)"$"#).unwrap();
    let value = re.captures(line).unwrap().at(1).unwrap();
    println!("{:?}", value);
    Ok(value.to_string())
}

fn get_channel(line: &str) -> Result<i32, Error> {
    let re = Regex::new(r"^Channel:(\d+)$").unwrap();
    let value_raw = re.captures(line).unwrap().at(1).unwrap();
    let value = try!(value_raw.parse::<i32>().map_err(|_| Error::FailedToParse));
    Ok(value)
}

fn main() {
    // println!("{:?}", get_signal("Signal level=58/100"));
    let text = "Signal level=58/100";
    assert_eq!(-71, get_signal(text).unwrap());
    println!("Signal strength: {:?}", get_signal_regex(text));

    let text = "ESSID:\"gsy-97796\"";
    println!("SSID: {:?}", get_ssid_regex(text));

    let text = "Channel:6";
    println!("Channel: {:?}", get_channel(text));
    // let foo = "Signal level=58/100".to_string();
    // let mut parts = foo.split("=").nth(1).unwrap().split("/");

    // for item in parts {
    //         println!("items: {:?}", item);
    // }

    // let value: i32 = parts.nth(0).unwrap().parse().unwrap();

    // let signal_strength : i32= (100 * value.parse().unwrap());
    // let signal_strength : i32 = 100 * value;
    // println!("signal_strength: {:?}", signal_strength);
    // println!("value {} maths {} ", value, ((100 * value) / 100) / 2 - 100);
    // println!("value: {} maths: {}", value, calc_signal(&value));
}

// #[allow(dead_code)]
// fn regex_wifi() {
//     // let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
//     // let text = "2012-03-14, 2013-01-01 and 2014-07-05";
//     // for cap in re.captures_iter(text) {
//     //     println!("Month: {} Day: {} Year: {}",
//     //              cap.at(2).unwrap_or(""),
//     //              cap.at(3).unwrap_or(""),
//     //              cap.at(1).unwrap_or(""));
//     // }
//
//     let cell_regex = Regex::new(r"Cell [0-9]{2,} - Address:").unwrap();
//     let mac_regex = Regex::new(r"([0-9a-zA-Z]{1}[0-9a-zA-Z]{1}[:]{1}){5}[0-9a-zA-Z]{1}[0-9a-zA-Z]{1}").unwrap();
//
//     let data = "wlan0     Scan completed :\n".to_string() +
//             "          Cell 01 - Address: D4:D1:84:50:76:45\n" +
//             "                    Channel:6\n" +
//             "                    Frequency:2.437 GHz (Channel 6)\n" +
//             "                    Quality=34/70  Signal level=-76 dBm\n" +
//             "                    Encryption key:on\n" +
//             "                    ESSID:\"gsy-97796\"\n" +
//             "                    Bit Rates:1 Mb/s; 2 Mb/s; 5.5 Mb/s; 11 Mb/s; 18 Mb/s\n" +
//             "                              24 Mb/s; 36 Mb/s; 54 Mb/s\n" +
//             "                    Bit Rates:6 Mb/s; 9 Mb/s; 12 Mb/s; 48 Mb/s\n" +
//             "                    Mode:Master\n" +
//             "                    Extra:tsf=0000000000000000\n" +
//             "                    Extra: Last beacon: 17180ms ago\n" +
//             "                    IE: Unknown: 00096773792D3937373936\n" +
//             "                    IE: Unknown: 010882848B962430486C\n" +
//             "          Cell 02 - Address: 7C:B7:33:AE:3B:04\n" +
//             "                    Channel:9\n" +
//             "                    Frequency:2.452 GHz (Channel 9)\n" +
//             "                    Quality=36/70  Signal level=-74 dBm\n" +
//             "                    Encryption key:on\n" +
//             "                    ESSID:\"Raupo\"\n" +
//             "                    Bit Rates:1 Mb/s; 2 Mb/s; 5.5 Mb/s; 11 Mb/s; 18 Mb/s\n" +
//             "                              24 Mb/s; 36 Mb/s; 54 Mb/s\n" +
//             "                    Bit Rates:6 Mb/s; 9 Mb/s; 12 Mb/s; 48 Mb/s\n" +
//             "                    Mode:Master\n" +
//             "                    Extra:tsf=0000000000000000\n" +
//             "                    Extra: Last beacon: 17180ms ago\n" +
//             "                    IE: Unknown: 0005526175706F\n" +
//             "                    IE: Unknown: 010882848B962430486C";
//
//         for block in cell_regex.split(&data) {
//             let mut lines = block.lines();
//
//     // mac
//             let  mac_matches = mac_regex.captures(lines.next().unwrap());
//     // println!("mac: {:?}", mac_matches);
//             if let Some(matches) = mac_matches {
//                 if let Some(mac) = matches.at(0) {
//                 println!("mac: {}", mac);
//                 }
//             }
//
//
//             for line in lines {
//                 if line.find("ESSID:").is_some() {
//                     let ssid = line.split(":").nth(1).unwrap_or("").replace("\"","");
//                     println!("SSID: {}", ssid);
//                 } else if line.find("Frequency:").is_some() {
//                     println!("Channel: {}", line.split("Channel").nth(1).unwrap_or("").replace(")","").trim());
//                 } else if line.find("Signal level").is_some() {
//
//
//     // case1
//                         println!("rssi (case 1): {}", line.split("Signal level=").nth(1).unwrap_or("").replace("dBm", "").trim());
//
//     // case 2
//                         println!("rssi: case 2 pending");
//                     }
//                 } else {
//     // println!(">> {}", line);
//                 }
//             } // for
//         }
