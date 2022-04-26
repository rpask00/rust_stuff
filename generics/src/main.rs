struct Point<T> {
    x: T,
    y: T,
}

impl Point<U> {
    fn xx(&self) -> U {
        self.x
    }
}

impl Point<f64> {
    fn yy(&self) -> f64 {
        self.y
    }
}


fn main() {
    let point = Point {
        x: 3,
        y: 3,
    };
    let point2 = Point {
        x: 3.3,
        y: 3.3,
    };

    point2.yy();
    point.yy();
}

