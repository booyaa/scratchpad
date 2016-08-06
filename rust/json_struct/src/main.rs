#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct GpsLocation {
    accuracy: u32,
    location: Location,
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Location {
    lat: f64,
    lng: f64,
}

fn main() {
    let point = Point { x: 1.0, y: 2.0 };

    let s = serde_json::to_string(&point).unwrap();
    assert_eq!(s, "{\"x\":1.0,\"y\":2.0}");

    let deserialized_point: Point = serde_json::from_str(&s).unwrap();
    assert_eq!(point, deserialized_point);

    let gps = GpsLocation {
        accuracy: 2000,
        location: Location {
            lat: 0.54,
            lng: -0.24,
        },
    };

    let s2 = serde_json::to_string(&gps).unwrap();
    assert_eq!(s2,
               "{\"accuracy\":2000,\"location\":{\"lat\":0.54,\"lng\":-0.24}}");

    let d2: GpsLocation = serde_json::from_str(&s2).unwrap();
    println!("{:?}", d2);
    // assert_eq!(s2, d2);
}
