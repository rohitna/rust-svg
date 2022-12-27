use geo::point;
use geo::prelude::*;
use geo::Line;
use geo::Point;
use geo_types::CoordFloat;
use svg::node::element::path::Data;
use svg::node::element::path::Parameters;
use svg::node::element::Circle;
use svg::node::element::Path;
use svg::node::Value;
use svg::Node;

type Canvas = svg::Document;

use crate::utils::polar_point;
use super::yantra_trait::{Config, LeafStyleDetailed, Yantra};

trait SetConfig<T: CoordFloat> {
    fn set_config(self, config: Config<T>) -> Self;
}

macro_rules! implement_set_config(
    ($struct_name:ident) => (
        impl<T> SetConfig<T> for $struct_name
        where
            T: CoordFloat,
            Value: From<T>,
            Parameters: From<T>,
        {
            #[inline]
            fn set_config(self, config: Config<T>) -> Self
            // where
            //     T: Into<Box<dyn crate::node::Node>>,
            {
                self
                .set("fill", config.fill_color.to_string())
                .set("stroke", config.stroke_color.to_string())
                .set("stroke-width", config.stroke_width)
            }
        }
    );
);

implement_set_config! {Circle}
implement_set_config! {Path}

/*====== Yantra Implementation for canvas ============ */
impl<T: CoordFloat> Yantra<T> for Canvas
where
    Value: From<T>,
    Parameters: From<T>,
{
    fn add_circle(self, radius: T, center: Point<T>, config: Config<T>) -> Canvas {
        let mut circle = Circle::new();
        circle.assign("cx", center.x());
        circle.assign("cy", center.y());
        circle.assign("r", radius);
        circle = circle.set_config(config);

        self.add(circle)
    }

    fn add_single_leaf(
        self,
        start: Point<T>,
        end: Point<T>,
        tip: Point<T>,
        leaf_style: LeafStyleDetailed<T>,
        config: Config<T>,
    ) -> Canvas {
        // Read the leaf style
        let mut radius = None;
        let (style, c1, c2, d1, d2) = match leaf_style {
            LeafStyleDetailed::Circular(c1, c2, d1, d2, r) => {
                radius = Some(r);
                ("circular", c1, c2, d1, d2)
            }
            LeafStyleDetailed::Linear(c1, c2, d1, d2) => ("linear", c1, c2, d1, d2),
            LeafStyleDetailed::Point(c1, c2, d1, d2) => {
                assert!(
                    start == end,
                    "start and end must be equal for point base type"
                );
                ("pointed", c1, c2, d1, d2)
            }
        };

        // Get control points for the leaf according to the style
        let zero = T::from(0.0).unwrap();
        let origin = Point::new(zero, zero);
        let mid = (start + end) / T::from(2.0).unwrap();
        let radial = tip - mid;
        let radial_length = Line::new(origin, radial).euclidean_length();
        let radial_direction = radial / radial_length;
        let transform = AffineTransform::rotate(T::from(90.0).unwrap(), origin);
        let perp_direction = radial_direction.affine_transform(&transform);
        let control_1 = start + radial_direction * c1 - perp_direction * c2;
        let control_2 = end + radial_direction * c1 + perp_direction * c2;
        let mid_control_1 = tip - radial_direction * d1 - perp_direction * d2;
        let mid_control_2 = tip - radial_direction * d1 + perp_direction * d2;

        // Create leaf data
        let leaf_data = Data::new()
            .move_to(start.x_y())
            .cubic_curve_to(match (control_1.x_y(), mid_control_1.x_y(), tip.x_y()) {
                ((a, b), (c, d), (e, f)) => (a, b, c, d, e, f),
            })
            .cubic_curve_to(match (mid_control_2.x_y(), control_2.x_y(), end.x_y()) {
                ((a, b), (c, d), (e, f)) => (a, b, c, d, e, f),
            });

        let leaf_data = match style {
            "circular" => leaf_data.elliptical_arc_by(match (start - end).x_y() {
                (a, b) => match radius {
                    Some(r) => (r, r, 0, 0, 0, a, b),
                    None => panic!("radius required!"),
                },
            }),
            _ => leaf_data.close(),
        };

        // Create a leaf using the data, and style it
        let leaf = Path::new().set_config(config).set("d", leaf_data);

        // Add the leaf to the yantra
        self.add(leaf)
    }

    /// Draw a polygon.
    /// This also supports just unclosed polygon (if the first point doesn't equal the last point).
    fn add_polygon(self, polygon: Vec<Point<T>>, config: Config<T>) -> Self {
        let start = polygon[0];
        let polygon_data = polygon[1..].into_iter()
        .fold(
            Data::new().move_to(start.x_y()),
            |polygon_data, point| polygon_data.line_to(point.x_y())
        );

        // Create a polygon path using the data, and style it
        let polygon_path = Path::new().set_config(config).set("d", polygon_data);

        // Add the ngon to the yantra
        self.add(polygon_path)
    }

    fn add_regular_n_gon(
        self,
        radius: T,
        center: Point<T>,
        alpha: T,
        n: usize,
        config: Config<T>,
    ) -> Self {
        assert!(n >= 3, "n gon must have at least three sides");
        let side_span = T::from(360.0).unwrap() / T::from(n).unwrap();
        let ngon_data = (1..=n)
            .map(|i| alpha + T::from(i).unwrap() * side_span)
            .fold(
                Data::new().move_to(polar_point(radius, center, alpha).x_y()),
                |ngon_data, cur_alpha| {
                    ngon_data.line_to(polar_point(radius, center, cur_alpha).x_y())
                },
            );

        // Create a ngon using the data, and style it
        let ngon = Path::new().set_config(config).set("d", ngon_data);

        // Add the ngon to the yantra
        self.add(ngon)
    }
}

