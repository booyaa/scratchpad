#![cfg_attr(feature = "serde_macros", feature(plugin, custom_derive))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate serde;
extern crate serde_json;

#[cfg(feature = "serde_macros")]
include!("serde_types.in.rs");

#[cfg(feature = "serde_codegen")]
include!(concat!(env!("OUT_DIR"), "/serde_types.rs"));

fn main() {
    let point = Point { x: 1.0, y: 2.0 };

    let s = serde_json::to_string(&point).unwrap();
    assert_eq!(s, "{\"x\":1.0,\"y\":2.0}");

    // let deserialized_point: Point = serde_json::from_str(&s).unwrap();

    let blah = "{\"a\": [ { \"result\" : 1 }, { \"topics\" : 2} ], \"Type\" : \"D\" }";
    let deserialized_point: serde_json::Value = serde_json::from_str(&blah).unwrap();
    println!("{:#?}", deserialized_point);

    let a = deserialized_point.as_object().unwrap().get("a").unwrap();
    println!("obj: {} arr: {}", a.is_object(), a.is_array());

    let check_type = deserialized_point.as_object().unwrap().get("Type").unwrap();

    let format = format!("{}", check_type);
    println!("check_type {}", format);


    if format == "D" {
        println!("uh oh!");
    } else {
        println!("nah");
    }

    // assert_eq!(point, deserialized_point);
}
