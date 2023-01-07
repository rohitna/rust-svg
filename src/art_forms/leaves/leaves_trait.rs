//! This file describes the `Leaves` trait
//! A leaf is drawn on a canvas.
//! Different leaf styles are supported.
use crate::art_forms::base_shapes::{Config, BaseShapes};
use derive_new::new;
use geo::point;
use geo::prelude::*;
use geo::Line;
use geo::Point;
use geo_types::CoordFloat;

// Here `T` quntifies the fraction of area to use.
#[derive(Clone, new)]
/// Leaves are cubic bezier curves where the parameters denote
/// size, scale, control_1, control_2.
pub enum LeafStyle<T: CoordFloat> {
    // Reniform leaf shape
    Reniform(T, T, T, T),
    // Cordate leaf shape
    Cordate(T, T, T, T),
}

/// Base of a leaf could be circular, linear, or a point.
/// First two coordinates determine how the start and end of the leaf look.
/// Third and the Fourth coordinates determine how the leaf tip looks.
/// Fifth coordinate is the radius in the circular base case.
#[derive(Clone, new)]
pub enum LeafStyleDetailed<T> {
    Circular(T, T, T, T, T),
    Linear(T, T, T, T),
    Point(T, T, T, T),
}



/// `Self` is consumed everywhere in order to enable a builder patter API.
pub trait Leaves<T>: BaseShapes<T>
where
    Self: Sized,
    T: CoordFloat
{
    fn add_single_leaf(
        self,
        start: Point<T>,
        end: Point<T>,
        tip: Point<T>,
        leaf_style: LeafStyleDetailed<T>,
        config: Config<T>,
    ) -> Self;

    // Add a linear leaf
    // The leaf points away from the origin as long as origin -> p -> q is counter-clockwise.
    fn add_single_linear_leaf(
        self,
        p: Point<T>,
        q: Point<T>,
        leaf_style: LeafStyle<T>,
        config: Config<T>,
    ) -> Self {
        // Read the leaf style
        let (size, scale, style, c, d) = match leaf_style {
            LeafStyle::Reniform(s, t, c, d) => (s, t, "reniform", c, d),
            LeafStyle::Cordate(s, t, c, d) => (s, t, "cordate", c, d),
        };

        // Calculate the start point, the end point, and the leaf tip
        let mid = (p + q) / T::from(2.0).unwrap();
        let start = mid - (mid - p) * scale;
        let end = mid + (q - mid) * scale;

        let zero = T::from(0.0).unwrap();
        let origin = Point::new(zero, zero);

        let orientation = origin.cross_prod(start, end) > zero;
        let transform = AffineTransform::rotate(T::from(90.0).unwrap(), origin);
        if !orientation {
            #[allow(unused_variables)]
            let (start, end) = (end, start);
        };
        let radial = (start - end).affine_transform(&transform);
        let radial_length = Line::new(origin, radial).euclidean_length();
        let radial_direction = radial / radial_length;
        let tip = mid + radial_direction * size;

        // Get detailed style
        let detailed_style = match style {
            "reniform" => LeafStyleDetailed::Linear(c, zero, zero, d),
            "cordate" => LeafStyleDetailed::Linear(c, zero, d, zero),
            &_ => todo!(),
        };

        self.add_single_leaf(start, end, tip, detailed_style, config)
    }

    /// Add a circular leaf to the yantra.
    /// Circular part should be inverted if `alpha > beta`.
    fn add_single_circular_leaf(
        self,
        radius: T,
        center: Point<T>,
        alpha: T,
        beta: T,
        leaf_style: LeafStyle<T>,
        config: Config<T>,
    ) -> Self {
        // Read the leaf style
        let (size, scale, style, c, d) = match leaf_style {
            LeafStyle::Reniform(s, t, c, d) => (s, t, "reniform", c, d),
            LeafStyle::Cordate(s, t, c, d) => (s, t, "cordate", c, d),
        };

        // Calculate the start point, the end point, and the leaf tip
        let mid = (beta + alpha) / T::from(2.0).unwrap();
        let alpha = mid - scale * (mid - alpha);
        let beta = mid + scale * (beta - mid);

        let (sin_alpha, cos_alpha) = alpha.to_radians().sin_cos();
        let (sin_beta, cos_beta) = beta.to_radians().sin_cos();
        let (sin_mid, cos_mid) = mid.to_radians().sin_cos();

        let start = center + point!(x: radius * cos_alpha, y: radius * sin_alpha);
        let end = center + point!(x: radius * cos_beta, y: radius * sin_beta);
        let tip = center + point!(x: (radius + size) * cos_mid, y: (radius + size) * sin_mid);

        // Get detailed style
        let (sin_half, cos_half) = (mid - alpha).to_radians().sin_cos();
        let (c1, c2) = (c * cos_half, c * sin_half);
        let zero = T::from(0.0).unwrap();
        let detailed_style = match style {
            "reniform" => LeafStyleDetailed::Circular(c1, c2, zero, d, radius),
            "cordate" => LeafStyleDetailed::Circular(c1, c2, d, zero, radius),
            &_ => todo!(),
        };

        self.add_single_leaf(start, end, tip, detailed_style, config)
    }

    // Add circular leaves on an arc
    fn add_circular_leaves_on_an_arc(
        self,
        radius: T,
        center: Point<T>,
        alpha: T,
        beta: T,
        num_leaves: usize,
        leaf_style: LeafStyle<T>,
        config: Config<T>,
    ) -> Self {
        assert!(num_leaves > 0, "num_leaves must be positive");
        let leaf_span = (beta - alpha) / T::from(num_leaves).unwrap();
        let yantra = (0..num_leaves)
            .map(|i| alpha + T::from(i).unwrap() * leaf_span)
            .fold(self, |yantra, cur_alpha| {
                yantra.add_single_circular_leaf(
                    radius,
                    center,
                    cur_alpha,
                    cur_alpha + leaf_span,
                    leaf_style.clone(),
                    config.clone(),
                )
            });

        yantra
    }

    // Add a circular leaf on a circle
    // Orientation is 1.0 for counterclockwise, -1.0 for clockwise.
    fn add_circular_leaves(
        self,
        radius: T,
        center: Point<T>,
        alpha: T,
        num_leaves: usize,
        orientation: T,
        leaf_style: LeafStyle<T>,
        config: Config<T>,
    ) -> Self {
        self.add_circular_leaves_on_an_arc(
            radius,
            center,
            alpha,
            alpha + orientation * T::from(360.0).unwrap(),
            num_leaves,
            leaf_style,
            config,
        )
    }
}