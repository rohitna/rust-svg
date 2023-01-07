use crate::art_forms::base_shapes::{BaseShapes, Config};
use crate::art_forms::leaves::{LeafStyle, Leaves};

use geo::point;
use svg::Document;

type Canvas = Document;
type T = f64;

#[test]
fn test_add_single_circular_leaf() {
    let view_size = 200.0;
    let canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 100.0;
    let config = Config::<T>::new(3.0, "blue".to_string(), "yellow".to_string());
    // add a circle, a cordate leaf, and two reniform leaf to the canvas
    let cordate_style = LeafStyle::<T>::new_cordate(40.0, 0.5, 24.0, 26.0);
    let reniform_style_1 = LeafStyle::<T>::new_reniform(40.0, 0.5, 24.0, 20.0);
    let reniform_style_2 = LeafStyle::<T>::new_reniform(40.0, 0.5, 20.0, 0.0);
    let canvas = canvas
        .add_circle(radius, center, config.clone())
        .add_single_circular_leaf(radius, center, 0.0, 90.0, reniform_style_1, config.clone())
        .add_single_circular_leaf(radius, center, 90.0, 180.0, cordate_style, config.clone())
        .add_single_circular_leaf(
            radius,
            center,
            180.0,
            270.0,
            reniform_style_2,
            config.clone(),
        );

    // Save the image
    svg::save("./unit_tests/leaves/add_single_circular_leaf.svg", &canvas).unwrap();
}

#[test]
fn test_add_single_linear_leaf() {
    let view_size = 200.0;
    let canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 100.0;
    let config = Config::<T>::new(3.0, "blue".to_string(), "yellow".to_string());
    // add a linear reniform leaf, and a circle to the canvas
    let reniform_style = LeafStyle::<T>::new_reniform(50.0, 0.5, 12.0, 39.0);
    let canvas = canvas
        .add_circle(radius, center, config.clone())
        .add_single_linear_leaf(
            point!(x: 0., y: radius),
            point!(x: radius, y: 0.),
            reniform_style.clone(),
            config.clone(),
        )
        .add_single_linear_leaf(
            point!(x: -radius, y: 0.),
            point!(x: 0., y: -radius),
            reniform_style,
            config.clone(),
        );

    // Save the image
    svg::save("./unit_tests/leaves/add_single_linear_leaf.svg", &canvas).unwrap();
}

#[test]
fn test_add_circular_leaves_on_an_arc() {
    let view_size = 200.0;
    let canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 100.0;
    let config = Config::<T>::new(3.0, "blue".to_string(), "yellow".to_string());
    // add a circle, and 10 cordate leaves in 0 - 180 degrees
    let cordate_style = LeafStyle::<T>::new_cordate(40.0, 1.0, 24.0, 20.0);
    let canvas = canvas
        .add_circle(radius, center, config.clone())
        .add_circular_leaves_on_an_arc(radius, center, 0.0, 270.0, 10, cordate_style, config);

    // Save the image
    svg::save(
        "./unit_tests/leaves/add_circular_leaves_on_an_arc.svg",
        &canvas,
    )
    .unwrap();
}

#[test]
fn test_add_circular_leaves_on_a_circle() {
    let view_size = 200.0;
    let canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 100.0;
    let config = Config::<T>::new(3.0, "blue".to_string(), "yellow".to_string());
    // add a circle, and 10 cordate leaves in 0 - 180 degrees
    let cordate_style = LeafStyle::<T>::new_cordate(40.0, 1.0, 62.0, 28.0);
    let canvas = canvas
        .add_circle(radius, center, config.clone())
        .add_circular_leaves(radius, center, 0.0, 10, -1.0, cordate_style, config);

    // Save the image
    svg::save("./unit_tests/leaves/add_circular_leaves.svg", &canvas).unwrap();
}
