use core::marker::PhantomData;
use core::ops::{Add, Sub, Mul, Div};

pub trait RealNumber: Sized + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> {
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;
    fn abs(self) -> Self;
}

impl RealNumber for f32 {
    const ZERO: Self = 0.0;
    const ONE: Self = 1.0;
    const TWO: Self = 2.0;
    fn abs(self) -> Self {
        Self::abs(self)
    }
}
pub trait Area2D<T: Copy + RealNumber, const N: usize> {
    fn area(x: [T; N], y: [T; N]) -> T {
        if N < 3 { return T::ZERO; }
        // Apply the Shoelace formula.
        // https://mathworld.wolfram.com/ShoelaceFormula.html
        (0..N)
            .map(|i| (i, (i + 1) % N))
            .fold(T::ZERO, |acc, (v1, v2)| {
                acc + (x[v1] * y[v2]) - (x[v2] * y[v1])
            })
            .abs()
            / T::TWO
    }
}

pub struct DefaultSubprograms;
impl<T: Copy + RealNumber, const N: usize> Area2D<T, N> for DefaultSubprograms {}

pub struct Polygon2D<T, const N: usize, A=DefaultSubprograms> {
    pub vertices_x: [T; N],
    pub vertices_y: [T; N],
    _phantom: PhantomData<A>,
}

impl<T, const N: usize, A> Polygon2D<T, N, A> {
    pub const fn new(vertices_x: [T; N], vertices_y: [T; N]) -> Self {
        Self {
            vertices_x,
            vertices_y,
            _phantom: PhantomData,
        }
    }
}

impl<T: Copy + RealNumber, const N: usize, A: Area2D<T, N>> Polygon2D<T, N, A> {
    pub fn area(&self) -> T {
        A::area(self.vertices_x, self.vertices_y)
    }
}

pub struct Squarea;
impl<T: Copy + RealNumber> Area2D<T, 4> for Squarea {
    fn area(xs: [T; 4], ys: [T; 4]) -> T {
        ((xs[0] * ys[1] - xs[1] * ys[0])
            + (xs[1] * ys[2] - xs[2] * ys[1])
            + (xs[2] * ys[3] - xs[3] * ys[2])
            + (xs[3] * ys[0] - xs[0] * ys[3])).abs()
            / T::TWO
    }
}
pub type Square<T, Area2D=Squarea> = Polygon2D<T, 4, Area2D>;

pub struct UnitShape<T>(PhantomData<T>);
impl<T: Copy + RealNumber, const N: usize> Area2D<T, N> for UnitShape<T> {
    fn area(_xs: [T; N], _ys: [T; N]) -> T {
        T::ONE
    }
}

#[cfg(test)]
mod square_tests {
    use super::*;

    #[test]
    #[allow(clippy::similar_names, clippy::float_cmp)]
    #[inline(never)]
    fn test_area() {
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
}