use polygon_rs::{DefaultSubprograms, Polygon2D, Square, UnitShape, Area2D};

fn main() {
    let cw_square: Square<f32> = Square::new(
        [1.0, 2.0, 2.0, 1.0],
        [1.0, 1.0, 0.0, 0.0]
    );
    let ccw_square: Square<f32, UnitShape<f32>> = Square::new(
        [0.0, 1.0, 1.0, 0.0],
        [0.0, 0.0, 1.0, 1.0]
    );
    let polygon: Polygon2D<f32, 4, DefaultSubprograms> = Polygon2D::new(
        [1.0, 2.0, 2.0, 1.0],
        [1.0, 1.0, 0.0, 0.0],
    );

    assert_eq!(cw_square.area(), ccw_square.area());
    assert_eq!(cw_square.area(), polygon.area());
    assert_eq!(cw_square.area(), UnitShape::area([0.0], [0.0]));
}