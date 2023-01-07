use geo::{point, Point, Line};
use geo::line_intersection::{line_intersection, LineIntersection};
use geo::prelude::*;
use geo_types::CoordFloat;
use geo::GeoFloat;
// use numeric_literals::replace_numeric_literals;

pub fn polar_point<T: CoordFloat>(radius: T, center: Point<T>, alpha: T) -> Point<T> {
    center
        + point!(
            x: radius * alpha.to_radians().cos(),
            y: radius * alpha.to_radians().sin()
        )
}

pub fn dist<T: CoordFloat>(p: Point<T>, q: Point<T>) -> T {
    Line::new(p, q).euclidean_length()
}

/// Get the intersection of lines p--q and r--s.
pub fn intersection<T: CoordFloat + GeoFloat>(p: Point<T>, q: Point<T>, r: Point<T>, s: Point<T>) -> Point<T> {
    let intersection = line_intersection(Line::new(p,q), Line::new(r,s));
    match intersection.expect("Intersection not found") {
        LineIntersection::SinglePoint{ intersection: point, ..}  => Point::from(point),
        LineIntersection::Collinear{ .. } => panic!("Intersection not found")
    }
}

// #[replace_numeric_literals(T::from(literal).unwrap())]
// pub fn golden_ratio<T: CoordFloat>() -> T {
//    (1 + 5.sqrt()) / 2
// }