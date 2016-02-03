use num::Float;

use {Yxy, flt};

// IS AN ENUM OF WHITE POINTS REQUIRED??
// pub enum WhitePointKind {
//     D65,
//     D65Fov10,
//     D50,
//     D50Fov10,
// }
//
// pub trait GetWhitePoint<T:Float>: Sized {
//     fn yxy(&self) -> Yxy<T>;
// }
//
// impl<T:Float> GetWhitePoint<T> for WhitePointKind {
//     fn get_yxy_vals(&self) -> Yxy<T> {
//         match *self {
//             WhitePointKind::D65 => D65::get_yxy(),
//             WhitePointKind::D65Fov10 => D65Fov10::get_yxy(),
//             WhitePointKind::D50 => D50::get_yxy(),
//             WhitePointKind::D50Fov10 => D50Fov10::get_yxy(),
//             WhitePointKind::NoWhitePoint => NoWhitePoint::get_yxy(),
//         }
//     }
// }

pub trait WhitePoint<T: Float> {
    fn get_yxy() -> Yxy<T>;
}

macro_rules! generate_white_point {
    ($x: ident => ($p1: expr, $p2:expr, $p3:expr)) => (
        impl<T: Float> WhitePoint<T> for $x {

            fn get_yxy() -> Yxy<T> {
                Yxy::new(flt($p1), flt($p2), flt($p3))
            }
        }
    );
}

// pub struct NoWhitePoint;
// generate_white_point!(NoWhitePoint => (1.0, 1.0, 1.0));

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D65;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D65Fov10;

generate_white_point!(D65 => (0.31271,0.32902, 1.0));
generate_white_point!(D65Fov10 => (0.34773, 0.35952, 1.0));

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D50;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct D50Fov10;

generate_white_point!(D50 => ( 0.31271,0.32902, 1.0));
generate_white_point!(D50Fov10 => (0.31382,0.33100, 1.0));
