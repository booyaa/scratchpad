// code from rbe
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    // associated functions
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle {
            x: x,
            y: y,
            radius: radius,
        }
    }
    // method calls
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }

    fn grow(&self, increment: f64) -> Circle {
        Circle {
            x: self.x,
            y: self.y,
            radius: self.radius + increment,
        }
    }

    // fn reference(&self) {
    //     println!("taking self by reference!");
    // }
    //
    // fn mutable_reference(&mut self) {
    //     println!("taking self by mutable reference!");
    // }
    //
    // fn takes_ownership(self) {
    //     println!("taking ownership of self!");
    // }
}

fn main() {
    let c = Circle::new(0.0, 0.0, 2.0);
    println!("c.area {}", c.area());

    let d = c.grow(2.0).area();
    println!("d.grow.area: {}", d);
}
