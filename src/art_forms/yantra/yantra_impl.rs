
use geo_types::CoordFloat;
use svg::node::element::path::Parameters;
use svg::node::Value;


type Canvas = svg::Document;

use super::yantra_trait::{Yantra};
use crate::art_forms::base_shapes::Config;


/*====== Yantra Implementation for canvas ============ */
impl<T: CoordFloat> Yantra<T> for Canvas
where
    Value: From<T>,
    Parameters: From<T>,
{
    fn add_sri(self, _radius: T, _center: geo::Point<T>, _alpha: T, _config: [Config<T>; 5]) -> Self {
        todo!();
    }
}
