#![feature(custom_derive)]

extern crate serde;
extern crate serde_json;

use serde_json::Value;

fn main() {
    let data: Value = serde_json::from_str("{
                           \"accuracy\" : 20,
                           \"location\" : {
                              \"lat\" : 51.8852769,
                              \"lng\" : 0.8930123
                           },
                           \"status\" : \"OK\"
                       }")
                          .unwrap();

    println!("data: {:?}", data);


    let obj = data.as_object().unwrap();
    let accuracy = obj.get("accuracy").unwrap();
    let location = obj.get("location").unwrap().as_object().unwrap();
    let lat = location.get("lat").unwrap();
    let lng = location.get("lng").unwrap();
    println!("accuracy: {:?} location: {:?} lat: {:?} lng: {:?}",
             accuracy,
             location,
             lat,
             lng);

}
