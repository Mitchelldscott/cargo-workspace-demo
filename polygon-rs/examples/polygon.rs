use polygon_rs::{Polygon, Vertex};

/// A simple struct representing a point in 2D space.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Point2D([f64; 2]);

/// Implementation of the `Vertex` trait for our `Point` struct.
impl Vertex for Point2D {
    fn x(&self) -> f64 {
        self.0[0]
    }
    fn y(&self) -> f64 {
        self.0[1]
    }
}

pub struct Triangle([Point2D; 3]);

impl Polygon<Point2D> for Triangle {
    fn vertices(&self) -> Vec<Point2D> {
        self.0.to_vec()
    }
}

pub struct Square([Point2D; 4]);

impl Polygon<Point2D> for Square {
    fn vertices(&self) -> Vec<Point2D> {
        self.0.to_vec()
    }
}

pub struct CCWNgon(Vec<Point2D>);

impl Polygon<Point2D> for CCWNgon {
    fn vertices(&self) -> Vec<Point2D> {
        self.0.to_vec()
    }
}

fn main() {
    let triangle = Triangle([
        Point2D([0.0, 0.0]),
        Point2D([1.0, 0.0]),
        Point2D([0.0, 1.0]),
    ]);
    assert_eq!(triangle.area(), 0.5, "Incorrect Triangle area");

    let square = Square([
        Point2D([0.0, 0.0]),
        Point2D([1.0, 0.0]),
        Point2D([1.0, 1.0]),
        Point2D([0.0, 1.0]),
    ]);
    assert_eq!(square.area(), 1.0, "Incorrect square area");

    let ngon = CCWNgon(vec![
        Point2D([0.0, 0.0]),
        Point2D([0.5, 0.25]),
        Point2D([1.0, 0.0]),
        Point2D([0.75, 0.5]),
        Point2D([1.0, 1.0]),
        Point2D([0.5, 0.75]),
        Point2D([0.0, 1.0]),
        Point2D([0.25, 5.0]),
    ]);
    // >>> 0.5 * 0.25
    // 0.125
    // >>> 0.125 * 4
    // 0.5
    assert_eq!(ngon.area(), 0.5, "Incorrect ngon area");
}
