use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn euclidean_distance(&self, other: &Point) -> f64 {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }

    fn is_collinear(&self, p2: &Point, p3: &Point) -> bool {
        let x = self.x * (p2.y - p3.y);
        let y = p2.x * (self.y - p3.y);
        let z = p3.x * (p2.y - self.y);
        (x + y + z) == 0.0
    }

    fn to_quad1(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    fn to_quad2(self) -> Self {
        Self {
            x: self.x.abs(),
            y: -self.y.abs(),
        }
    }

    fn to_quad3(self) -> Self {
        Self {
            x: -self.x.abs(),
            y: -self.y.abs(),
        }
    }

    fn to_quad4(self) -> Self {
        Self {
            x: -self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Div for Point {
    type Output = Point;

    fn div(self, other: Point) -> Point {
        Point {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

fn main() {
    let p1 = Point::new(1.0, 2.0);
    let p2 = Point::new(2.0, 4.0);
    let p3 = Point { x: 4.0, y: 6.0 };
    let _origin = Point::origin();

    // Points addition
    println!("> Addition: {} + {} = {}", p1, p2, p1.clone() + p2.clone());

    // Calculate euclidean distance
    println!(
        "> Euclidean Distance between {} and {} is {}",
        p3,
        p1,
        p3.euclidean_distance(&p1),
    );

    // Collinear points
    println!(
        "> Are Points {}, {} and {} are collinear? {}",
        p1,
        p2,
        p3,
        match p1.is_collinear(&p2, &p3) {
            true => "Yes",
            false => "No",
        }
    );

    // Move to 1st Quadrant
    let temp_point = Point::new(-1.8, -4.9);
    println!(
        "> Move point {} to 1st quadrant: {}",
        temp_point,
        temp_point.clone().to_quad1()
    );
    println!(
        "> Move point {} to 2nd quadrant: {}",
        temp_point,
        temp_point.clone().to_quad2()
    );
    println!(
        "> Move point {} to 3rd quadrant: {}",
        temp_point,
        temp_point.clone().to_quad3()
    );
    println!(
        "> Move point {} to 4th quadrant: {}",
        temp_point,
        temp_point.clone().to_quad4()
    );
}
