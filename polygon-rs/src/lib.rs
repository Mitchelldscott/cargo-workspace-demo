//! # Crate-level documentation
//! _This will appear on the main page of the docs_
//!
//! A text file can be included in the docs using `#![doc = include_str!("../../README.md")]`
#![doc = include_str!("../../README.md")]

#![no_std]

// Clippy docs: https://doc.rust-lang.org/clippy/usage.html
#![deny(
    clippy::all,
    clippy::todo,
    clippy::panic,
    clippy::style,
    clippy::nursery,
    clippy::pedantic,
    clippy::suspicious,
    clippy::complexity,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::unimplemented
)]
#![warn(unused, rust_2018_idioms)]

pub trait Vertex {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

impl<T> Vertex for [T; 2] 
where 
    T: Copy,
    f32: From<T>,
{
    fn x(&self) -> f32 { self[0].into() }
    fn y(&self) -> f32 { self[1].into() }
}

impl<T> Vertex for (T, T)
where 
    T: Copy,
    f32: From<T>,
{
    fn x(&self) -> f32 { self.0.into() }
    fn y(&self) -> f32 { self.1.into() }
}

pub trait Polygon {
    type V: Vertex;
    fn vertices(&self) -> &[Self::V];
    fn area(&self) -> f32 {
        let vertices = self.vertices();
        if vertices.len() < 3 { return 0.0; }
        // Apply the Shoelace formula.
        // https://mathworld.wolfram.com/ShoelaceFormula.html
        (0..vertices.len())
            .map(|i| (&vertices[i], &vertices[(i + 1) % vertices.len()]))
            .fold(0.0, |acc, (v1, v2)| {
                acc + (v1.x() * v2.y()) - (v1.y() * v2.x())
            })
            .abs()
            / 2.0
    }
}

// #[cfg(test)]
// mod ngon_test {
//     use super::*;

//     pub struct CCWNgon<const N: usize>([[f32; 2]; N]);

//     impl<const N: usize> Polygon for CCWNgon<N> {
//         type V = [f32; 2];
//         fn vertices(&self) -> &[Self::V] {
//             &self.0
//         }
//     }

//     pub struct CWNgon<const N: usize>([(f32, f32); N]);

//     impl<const N: usize> Polygon for CWNgon<N> {
//         type V = (f32, f32);
//         fn vertices(&self) -> &[Self::V] {
//             &self.0
//         }
//     }

//     #[test]
//     fn ccw_vs_cw() {
//         let ccw_ngon = CCWNgon([
//             [0.0, 0.0],
//             [0.5, 0.25],
//             [1.0, 0.0],
//             [0.75, 0.5],
//             [1.0, 1.0],
//             [0.5, 0.75],
//             [0.0, 1.0],
//             [0.25, 5.0],
//         ]);
//         let cw_ngon = CWNgon([
//             (0.0, 0.0),
//             (0.25, 5.0),
//             (0.0, 1.0),
//             (0.5, 0.75),
//             (1.0, 1.0),
//             (0.75, 0.5),
//             (1.0, 0.0),
//             (0.5, 0.25),
//         ]);
//         assert_eq!(ccw_ngon.area(), cw_ngon.area(), "Incorrect ngon area");
//     }
// }
