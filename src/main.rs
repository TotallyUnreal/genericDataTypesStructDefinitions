struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 10.0, y: 19.9 };
    //    let wont_work = Point { x: 25, y: 54.5 } // Needs to be the same datatype if adding
    // data to same generic type
}
