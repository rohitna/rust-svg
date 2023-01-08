
use geo::GeoFloat;
use geo_types::CoordFloat;
use svg::node::element::path::Parameters;
use svg::node::Value;
use geo::Point;


type Canvas = svg::Document;

use super::yantra_trait::{Yantra};
use super::sri_yantra_geometry::ShriYantra;
use crate::art_forms::base_shapes::{Config, BaseShapes};


/*====== Yantra Implementation for canvas ============ */
impl<T: CoordFloat> Yantra<T> for Canvas
where
    Value: From<T>,
    Parameters: From<T>,
    T: GeoFloat,
{
    fn add_sri(self, radius: T, center: Point<T>, config: [Config<T>; 9]) -> Self {
        let mut sri = ShriYantra::new(
            radius,
            center,
            None,
            None,
            None,
            None,
            None,
        );
        sri.construct_all_points();

    let first_outer = sri.first_outer_path();
    let first_inner = sri.first_inner_path();
    let second_outer = sri.second_outer_path();
    let second_inner = sri.second_inner_path();
    let third_outer = sri.third_outer_path();
    let third_inner = sri.third_inner_path();
    let fourth_outer = sri.fourth_outer_path();
    let fourth_inner = sri.fourth_inner_path();
    let fifth_outer = sri.fifth_outer_path();

    self
    .add_polygon(first_outer, config[0].clone())
    .add_polygon(first_inner, config[1].clone())
    .add_polygon(second_outer, config[2].clone())
    .add_polygon(second_inner, config[3].clone())
    .add_polygon(third_outer, config[4].clone())
    .add_polygon(third_inner, config[5].clone())
    .add_polygon(fourth_outer, config[6].clone())
    .add_polygon(fourth_inner, config[7].clone())
    .add_polygon(fifth_outer, config[8].clone())
    }
}
