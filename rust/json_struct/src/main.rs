#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate json;

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

    let parsed = json::parse(r#"

{
    "code": 200,
    "success": true,
    "payload": {
        "features": [
            "awesome",
            "easyAPI",
            "lowLearningCurve"
        ]
    }
}

"#)
                     .unwrap();

    //     let parsed = json::parse(r#"{\"crs\":{\"type\":\"link\",\"properties\":{\"href\":\"http:\\/\\/spatialreference.org\\/ref\\/epsg\\/4326\\/ogcwkt\\/\",\"type\":\"ogcwkt\"}},\"words\":\"index.home.r
    // aft\",\"bounds\":{\"southwest\":{\"lng\":-0.203607,\"lat\":51.521238},\"northeast\":{\"lng\":-0.203564,\"lat\":51.521265}},\"geometry\":{\"lng\":-0.203586,\"lat\":51.52
    // 1251},\"language\":\"en\",\"map\":\"http:\\/\\/w3w.co\\/index.home.raft\",\"status\":{\"status\":200,\"reason\":\"OK\"},\"thanks\":\"Thanks from all of us at index.home
    // .raft for using a what3words API\"}{\"crs\":{\"type\":\"link\",\"properties\":{\"href\":\"http:\\/\\/spatialreference.org\\/ref\\/epsg\\/4326\\/ogcwkt\\/\",\"type\":\"o
    // gcwkt\"}},\"words\":\"index.home.raft\",\"bounds\":{\"southwest\":{\"lng\":-0.203607,\"lat\":51.521238},\"northeast\":{\"lng\":-0.203564,\"lat\":51.521265}},\"geometry\
    // ":{\"lng\":-0.203586,\"lat\":51.521251},\"language\":\"en\",\"map\":\"http:\\/\\/w3w.co\\/index.home.raft\",\"status\":{\"status\":200,\"reason\":\"OK\"},\"thanks\":\"T
    // hanks from all of us at index.home.raft for using a what3words API\"}"#).unwrap();

    let parsed = json::parse(r#"
{
   "geometry" : {
      "lat" : 51.521251,
      "lng" : -0.203586
   },
   "thanks" : "Thanks from all of us at index.home.raft for using a what3words API",
   "language" : "en",
   "status" : {
      "status" : 200,
      "reason" : "OK"
   },
   "map" : "http://w3w.co/index.home.raft",
   "bounds" : {
      "southwest" : {
         "lng" : -0.203607,
         "lat" : 51.521238
      },
      "northeast" : {
         "lat" : 51.521265,
         "lng" : -0.203564
      }
   },
   "crs" : {
      "type" : "link",
      "properties" : {
         "type" : "ogcwkt",
         "href" : "http://spatialreference.org/ref/epsg/4326/ogcwkt/"
      }
   },
   "words" : "index.home.raft"
}
"#)
                     .unwrap();
    println!("{:?}", parsed);
    // println!("{}", parsed["code"]);
    // println!("{}", parsed["success"]);
    // for item in parsed["payload"]["features"].members() {
    // println!("{:?}", item);
    // }
}
