struct Point<T> {
    x: T,
    y: T,
}

struct PointMuliGenerics<T, U> {
    x: T,
    y: U,
}

// Using a different generic name but using the same name is conventional
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// This function is only avaliable for a point struct that holds f32 datatypes other methods can be
// implemented for different datatypes and overriding functions for different datatypes is also
// allowed
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        println!("Overridden function for different datatypes!");
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float: Point<f32> = Point { x: 10.0, y: 19.9 };
    let float_f64: Point<f64> = Point {
        x: 1350918.13541,
        y: 2342492.53287246,
    };
    //    let wont_work = Point { x: 25, y: 54.5 } // Needs to be the same datatype if adding
    // data to same generic type
    let will_work = PointMuliGenerics { x: 6.4, y: 5 };
    println!(
        "integer point x: {}, float point x: {}",
        integer.x(),
        float.x()
    );
    println!(
        "Distance from origin calculation only avaliable on point with float type: {}",
        float.distance_from_origin()
    );
    println!(
        "Overridden distance_from_origin function for f64 point structure: {}",
        float_f64.distance_from_origin()
    );
}
