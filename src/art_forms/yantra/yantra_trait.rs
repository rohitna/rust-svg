//! This file describes the yantra trait
//! A yantra is drawn on a canvas

use crate::art_forms::base_shapes::{BaseShapes, Config};
use crate::art_forms::leaves::Leaves;
use geo::Point;
use geo_types::CoordFloat;

/// `Self` is consumed everywhere in order to enable a builder patter API.
pub trait Yantra<T>: BaseShapes<T> + Leaves<T>
where
    Self: Sized,
    T: CoordFloat,
{
    fn add_sri(self, radius: T, center: Point<T>, alpha: T, config: [Config<T>; 5]) -> Self;
}
