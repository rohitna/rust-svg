use crate::utils::dist;
use crate::utils::intersection;
use crate::utils::polar_point;
use derive_getters::Getters;
use derive_new::new;
use geo::GeoFloat;
use geo::Point;
use geo::Polygon;
use num::integer;
use svg::node::element::Circle;
use svg::node::element::Path;
use svg::node::Value;
use svg::node::element::path::Parameters;
// use geo::GeoFloat;
use geo_types::CoordFloat;

#[derive(Clone, Getters, new)]
pub struct Config<T: CoordFloat> {
    pub stroke_width: T,
    pub stroke_color: String,
    pub fill_color: String,
}

pub trait SetConfig<T: CoordFloat> {
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
                .set("stroke-linejoin", "round")
                .set("transform", "scale(1,-1)")
            }
        }
    );
);

implement_set_config! {Circle}
implement_set_config! {Path}

/// `Self` is consumed everywhere in order to enable a builder pattern API.
pub trait BaseShapes<T: CoordFloat>
where
    Self: Sized,
{
    /// Add a circle to the yantra.
    fn add_circle(self, radius: T, center: Point<T>, config: Config<T>) -> Self;

    fn add_line_string(self, polygon: Vec<Point<T>>, config: Config<T>, should_close: bool)
        -> Self;

    fn add_circles(self, radius: T, centers: Vec<Point<T>>, config: Config<T>) -> Self {
        let yantra = centers.into_iter().fold(self, |yantra, point| {
            yantra.add_circle(radius, point, config.clone())
        });
        yantra
    }

    /// Polygon closed
    fn add_polygon(self, polygon: Vec<Point<T>>, config: Config<T>) -> Self {
        self.add_line_string(polygon, config, true)
    }

    /// Geo polygon is always closed
    fn add_geo_polygon(self, polygon: Polygon<T>, config: Config<T>) -> Self {
        let point_vec = polygon.into_inner().0.into_points();
        self.add_polygon(point_vec, config)
    }

    /// Add a regular n-gon
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
        let polygon = (0..n)
            .map(|i| alpha + T::from(i).unwrap() * side_span)
            .map(|cur_alpha| polar_point(radius, center, cur_alpha))
            .collect();
        self.add_polygon(polygon, config)
    }

    /// Add a [Star polygon](https://en.wikipedia.org/wiki/Star_polygon)
    fn add_star_polygon(
        self,
        radius: T,
        center: Point<T>,
        alpha: T,
        p: usize,
        q: usize,
        config: Config<T>,
    ) -> Self {
        assert!(q > 0, "q > 0 in order to avoid infinite loops.");
        assert!(p >= 3, "a polygon must have at least three vertices");
        assert!(integer::gcd(p, q) == 1, "invalid input to star polygon");
        let side_span = T::from(360.0).unwrap() / T::from(p).unwrap();
        let polygon: Vec<Point<T>> = (0..p)
            .map(|i| alpha + T::from(i).unwrap() * side_span)
            .map(|cur_alpha| polar_point(radius, center, cur_alpha))
            .collect();

        // let start = polar_point(radius, center, cur_alpha);
        let mut idx = q;
        let mut star = vec![polygon[0]];
        while polygon[idx] != polygon[0] {
            star.push(polygon[idx]);
            idx += q;
            idx %= p;
        }

        self.add_polygon(star, config)
    }

    fn add_isotoxal_star(
        self,
        radius: T,
        center: Point<T>,
        alpha: T,
        p: usize,
        q: usize,
        config: Config<T>,
        deformation: Option<usize>,
    ) -> Self
    where
        T: GeoFloat,
    {
        let remainder = q % p;
        assert!(
            remainder > 0 && remainder != 1 && remainder != p - 1,
            "Condition on remainder needed for a concave shape."
        );
        assert!(p >= 3, "a polygon must have at least three vertices");
        assert!(integer::gcd(p, q) == 1, "invalid input to star polygon");
        let side_span = T::from(360.0).unwrap() / T::from(p).unwrap();
        let start = polar_point(radius, center, alpha);
        let mid = polar_point(radius, center, alpha + side_span / T::from(2.0).unwrap());
        let first_edge_to = polar_point(radius, center, alpha + T::from(q).unwrap() * side_span);
        let intersection = intersection(start, first_edge_to, center, mid);
        let smaller_radius = dist(center, intersection);

        let star = (0..2 * p)
            .map(|i| match i % 2 {
                0 => polar_point(radius, center, alpha + T::from(i / 2).unwrap() * side_span),
                1 => polar_point(
                    smaller_radius,
                    center,
                    alpha
                        + side_span / T::from(2.0).unwrap()
                        + T::from((i + deformation.unwrap_or(0)) / 2).unwrap() * side_span,
                ),
                _ => panic!("I don't understand this remainder!"),
            })
            .collect();

        self.add_polygon(star, config)
    }
}
