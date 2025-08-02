//! # Crate-level documentation
//! _This will appear on the main page of the docs_
//!
//! A text file can be included in the docs using `#![doc = include_str!("../../README.md")]`
#![doc = include_str!("../../README.md")]

/// # Function level documentation
/// _This shows how to write a doc-test/example_
///
/// # Example
/// ```
/// use polygon_rs::Vertex;
/// struct Point2D(f64, f64);
/// impl Vertex for Point2D {
///     fn x(&self) -> f64 { self.0 }
///     fn y(&self) -> f64 { self.1 }
/// }
/// ```
pub trait Vertex {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}
pub trait Polygon<V: Vertex> {
    fn vertices(&self) -> Vec<V>;
    /// Find the area of a polygon
    fn area(&self) -> f64 {
        let vertices = self.vertices();
        if vertices.len() < 3 {
            return 0.0;
        }

        // Apply the Shoelace formula.
        // https://mathworld.wolfram.com/ShoelaceFormula.html
        let (sum1, sum2) = (0..vertices.len())
            .map(|i| (&vertices[i], &vertices[(i + 1) % vertices.len()]))
            .fold((0.0, 0.0), |(acc1, acc2), (v1, v2)| {
                (acc1 + (v1.x() * v2.y()), acc2 + (v1.y() * v2.x()))
            });

        0.5 * (sum1 - sum2).abs()
    }
}

impl<T> Vertex for [T; 2]
where
    T: Copy,
    f64: From<T>,
{
    fn x(&self) -> f64 {
        self[0].into()
    }
    fn y(&self) -> f64 {
        self[1].into()
    }
}

#[cfg(test)]
mod ngon_test {
    use super::*;

    pub struct CCWNgon(Vec<[f32; 2]>);

    impl Polygon<[f32; 2]> for CCWNgon {
        fn vertices(&self) -> Vec<[f32; 2]> {
            self.0.to_vec()
        }
    }

    pub struct CWNgon(Vec<[f32; 2]>);

    impl Polygon<[f32; 2]> for CWNgon {
        fn vertices(&self) -> Vec<[f32; 2]> {
            self.0.to_vec()
        }
    }

    #[test]
    fn ccw_vs_cw() {
        let ccw_ngon = CCWNgon(vec![
            [0.0, 0.0],
            [0.5, 0.25],
            [1.0, 0.0],
            [0.75, 0.5],
            [1.0, 1.0],
            [0.5, 0.75],
            [0.0, 1.0],
            [0.25, 5.0],
        ]);
        let cw_ngon = CWNgon(vec![
            [0.0, 0.0],
            [0.25, 5.0],
            [0.0, 1.0],
            [0.5, 0.75],
            [1.0, 1.0],
            [0.75, 0.5],
            [1.0, 0.0],
            [0.5, 0.25],
        ]);
        // >>> 0.5 * 0.25
        // 0.125
        // >>> 0.125 * 4
        // 0.5
        assert_eq!(ccw_ngon.area(), cw_ngon.area(), "Incorrect ngon area");
    }
}
