use geo::Point;
use geo_types::CoordFloat;
use svg::node::element::path::Data;
use svg::node::element::path::Parameters;
use svg::node::element::Circle;
use svg::node::element::Path;
use svg::node::Value;
use svg::Node;

type Canvas = svg::Document;

use super::base_shapes_trait::{Config, SetConfig, BaseShapes};



/*====== BaseShapes Implementation for canvas ============ */
impl<T: CoordFloat> BaseShapes<T> for Canvas
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

    /// Draw a polygon.
    /// This also supports just unclosed polygon (if the first point doesn't equal the last point).
    fn add_line_string(
        self,
        polygon: Vec<Point<T>>,
        config: Config<T>,
        should_close: bool,
    ) -> Self {
        let start = polygon[0];
        let polygon_data = polygon[1..]
            .into_iter()
            .fold(Data::new().move_to(start.x_y()), |polygon_data, point| {
                polygon_data.line_to(point.x_y())
            });

        // Create a polygon path using the data, and style it
        let polygon_path = Path::new().set_config(config).set(
            "d",
            match should_close {
                false => polygon_data,
                true => polygon_data.close(),
            },
        );

        // Add the ngon to the yantra
        self.add(polygon_path)
    }
}
