#![no_std]
#![no_main]

use panic_halt as _;

use polygon_rs::Polygon;

type Point2D = (f32, f32);

pub struct Triangle([Point2D; 3]);

impl Polygon for Triangle {
    type V = Point2D;
    fn vertices(&self) -> &[Point2D] { &self.0 }
}

// pub struct Square([Point2D; 4]);

// impl Polygon for Square {
//     type V = Point2D;
//     fn vertices(&self) -> &[Point2D] { &self.0 }
// }

// pub struct CCWNgon<const N: usize>([Point2D; N]);

// impl<const N: usize> Polygon for CCWNgon<N> {
//     type V = Point2D;
//     fn vertices(&self) -> &[Point2D] { &self.0 }
// }

fn main() {
    let triangle = Triangle([
        (0.0, 0.0),
        (1.0, 0.0),
        (0.0, 1.0),
    ]);
    // assert_eq!(triangle.area(), 0.5, "Incorrect Triangle area");

    // let square = Square([
    //     (0.0, 0.0),
    //     (1.0, 0.0),
    //     (1.0, 1.0),
    //     (0.0, 1.0),
    // ]);
    // assert_eq!(square.area(), 1.0, "Incorrect square area");

    // let ngon = CCWNgon([
    //     (0.0, 0.0),
    //     (0.5, 0.25),
    //     (1.0, 0.0),
    //     (0.75, 0.5),
    //     (1.0, 1.0),
    //     (0.5, 0.75),
    //     (0.0, 1.0),
    //     (0.25, 0.5),
    // ]);
    // // >>> 0.5 * 0.25
    // // 0.125
    // // >>> 0.125 * 4
    // // 0.5
    // assert_eq!(ngon.area(), 0.5, "Incorrect ngon area");
    let _a = triangle.area();
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    main();
    loop {}
}