use geo::{point, Point};
use geo_types::CoordFloat;

pub(crate) fn polar_point<T: CoordFloat>(radius: T, center: Point<T>, alpha: T) -> Point<T> {
    center
        + point!(
            x: radius * alpha.to_radians().cos(),
            y: radius * alpha.to_radians().sin()
        )
}