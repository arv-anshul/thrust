use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

trait Coordinate:
    fmt::Display
    + Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + PartialEq
{
    fn zero() -> Self;
    fn powi(self, n: i32) -> Self;
    fn sqrt(self) -> Self;
    fn abs(self) -> Self;
}

impl Coordinate for f32 {
    fn zero() -> Self {
        0.0_f32
    }

    fn powi(self, n: i32) -> Self {
        f32::powi(self, n)
    }

    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }

    fn abs(self) -> Self {
        f32::abs(self)
    }
}

impl Coordinate for f64 {
    fn zero() -> Self {
        0.0_f64
    }

    fn powi(self, n: i32) -> Self {
        f64::powi(self, n)
    }

    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }

    fn abs(self) -> Self {
        f64::abs(self)
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Point<T: Coordinate> {
    x: T,
    y: T,
}

impl<T: Coordinate> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<Point<f32>> for Point<f64> {
    /// Converts Point<f32> to Point<f64>
    fn from(val: Point<f32>) -> Self {
        Point {
            // Use the into() method from the f32 implementation above
            x: val.x.into(),
            y: val.y.into(),
        }
    }
}

impl<T: Coordinate> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

    pub fn origin() -> Point<T> {
        Point {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn euclidean_distance(&self, other: &Point<T>) -> T {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx.powi(2) + dy.powi(2)).sqrt()
    }

    pub fn is_collinear(&self, p2: &Point<T>, p3: &Point<T>) -> bool {
        let x = self.x * (p2.y - p3.y);
        let y = p2.x * (self.y - p3.y);
        let z = p3.x * (p2.y - self.y);
        (x + y + z) == T::zero()
    }

    pub fn to_quad1(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn to_quad2(self) -> Self {
        Self {
            x: self.x.abs(),
            y: -self.y.abs(),
        }
    }

    pub fn to_quad3(self) -> Self {
        Self {
            x: -self.x.abs(),
            y: -self.y.abs(),
        }
    }

    pub fn to_quad4(self) -> Self {
        Self {
            x: -self.x.abs(),
            y: self.y.abs(),
        }
    }
}

impl<T: Coordinate> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Coordinate> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T: Coordinate> Mul for Point<T> {
    type Output = Point<T>;

    fn mul(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T: Coordinate> Div for Point<T> {
    type Output = Point<T>;

    fn div(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

fn main() {
    // Example of using the Point struct
    let p1 = Point {
        x: 3.5f32,
        y: 4.2f32,
    };
    let p2 = Point::new(23.0, 1.1);

    println!("Origin Point: {}", Point::<f64>::origin());
    println!("Point 1: {}", p1);
    println!("Point 2: {}", p2);

    // Arithmetic operations on Points
    println!();
    println!("> Add: {} + {} = {}", p1, p2, p1 + p2);
    println!("> Sub: {} - {} = {}", p1, p2, p1 - p2);
    println!("> Mul: {} * {} = {}", p1, p2, p1 * p2);
    println!("> Div: {} ÷ {} = {}", p1, p2, p1 / p2);

    // Calculate euclidean distance
    println!();
    println!(
        "> Euclidean Distance between {} and {} is {}",
        p2,
        p1,
        p2.euclidean_distance(&p1),
    );

    // Move to different Quadrants
    println!();
    let p3 = Point::new(-1.8, -4.9);
    println!("> Move point {} to 1st quadrant: {}", p3, p3.to_quad1());
    println!("> Move point {} to 2nd quadrant: {}", p3, p3.to_quad2());
    println!("> Move point {} to 3rd quadrant: {}", p3, p3.to_quad3());
    println!("> Move point {} to 4th quadrant: {}", p3, p3.to_quad4());

    // Check collinear
    println!();
    println!(
        "> Are collinear {} - {} - {}?: {}",
        p1,
        p2,
        p3,
        p1.is_collinear(&p2, &p3)
    );
}
