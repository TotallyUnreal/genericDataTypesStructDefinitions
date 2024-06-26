#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
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

impl<X1, Y1> PointMuliGenerics<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointMuliGenerics<X2, Y2>) -> PointMuliGenerics<X1, Y2> {
        PointMuliGenerics {
            x: self.x,
            y: other.y,
        }
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

    let getting_crazy_with_it = PointMuliGenerics { x: 'x', y: 'y' };

    println!(
        "PointMuliGenerics mixing generics result: {:?}",
        will_work.mixup(getting_crazy_with_it)
    );
    // Note generics will not slow your code becuase of Monomorphization. Meaning the compiler
    // looks and generates chunks of code for each specific type it sees the user has it there
    // program. This is great but means you cannot rely on it when taking user input becuase no
    // types are given at compile time. The user could enter a integer float, String etc... at
    // least that is my understanding. With rusts strict type system it may be fine becuase you
    //                                                                 _---_
    // will be specifing the type the user will use for input anyway. /().()\
    //                                                                |  ^  |
    //                                                                 \   /
    //                                                                      <-- Owl Feet go here
}
