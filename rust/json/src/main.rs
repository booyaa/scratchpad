#![feature(custom_derive)]

extern crate serde;
extern crate serde_json;

use serde_json::Value;


#[derive(Debug, PartialEq)]
pub enum Error {
    /// A JSON parsing error occurred.
    JSON,
    NotAnObject,
    MissingValue,
    ParseIntError(std::num::ParseIntError),
    ParseFloatError(std::num::ParseFloatError),
}

// mad props to misdreavus for showing me the light
impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseIntError(err)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Self {
        Error::ParseFloatError(err)
    }
}

#[derive(Debug, PartialEq)]
struct GpsLocation {
    accuracy: u32,
    location: Location,
}

#[derive(Debug, PartialEq)]
struct Location {
    lat: f64,
    lng: f64,
}
fn parse(raw: String) -> Result<GpsLocation, Error> {
    let json_resp: Value = try!(serde_json::from_str(&raw).map_err(|_| Error::JSON));
    let json_obj = try!(json_resp.as_object().ok_or(Error::NotAnObject));

    let accuracy = try!(try!(json_obj.get("accuracy").ok_or(Error::MissingValue))
                            .to_string()
                            .parse::<u32>());
    let location = try!(json_obj.get("location").ok_or(Error::MissingValue)); // gets object?
    let location_object = try!(location.as_object().ok_or(Error::NotAnObject));
    let lat = try!(try!(location_object.get("lat").ok_or(Error::MissingValue))
                       .to_string()
                       .parse::<f64>());
    let lng = try!(try!(location_object.get("lng").ok_or(Error::MissingValue))
                       .to_string()
                       .parse::<f64>());

    Ok(GpsLocation {
        accuracy: accuracy,
        location: Location {
            lat: lat,
            lng: lng,
        },
    })
}

fn main() {
    println!("{:?}",
             parse("{\"accuracy\" : 20, \"location\" : { \"lat\" : 51.8852769, \"lng\" : \
                    0.8930123 }, \"status\" : \"OK\" }"
                       .to_string()));
}
