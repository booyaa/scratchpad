#[allow(unsed_imports)]
use serde::{de, Deserializer};
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: f64,
    y: f64,
}
