// use super::Yantra;
use crate::art_forms::base_shapes::{BaseShapes, Config};

use crate::art_forms::yantra::sri_yantra_geometry::ShriYantra;
use geo::point;
use svg::Document;

type Canvas = Document;
type T = f64;

#[test]
fn test_sri_points() {
    let view_size = 100.0;
    let canvas: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 100.0;
    let config = Config::<T>::new(1.0, "yellow".to_string(), "yellow".to_string());
    let canvas = canvas.add_circle(radius, center, config);

    let diameter = radius * 2.0;
    let mut sri = ShriYantra::new(
        radius,
        center,
        Some(diameter * 6.0 / 48.0),  // 1
        Some(diameter * 17.0 / 48.0), // 2
        Some(diameter * 27.0 / 48.0), // 3
        Some(diameter * 30.0 / 48.0), // 4
        Some(diameter * 42.0 / 48.0), // 5 , Tip of down triangle on the first) base?
    );

    let point_config = Config::<T>::new(1.0, "black".to_string(), "none".to_string());
    sri.construct_all_points();
    // let canvas = canvas.add_circle(1.0, ut_1, point_config.clone());
    let points = sri.get_all_points();
    let canvas = canvas.add_circles(0.5, points, point_config);

    svg::save("./unit_tests/yantra/add_sri_points.svg", &canvas).unwrap();
}

#[test]
fn test_sri_canvas() {
    let view_size = 100.0;
    let canvas: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 100.0;
    let config = Config::<T>::new(1.0, "yellow".to_string(), "yellow".to_string());
    let canvas = canvas.add_circle(radius, center, config.clone());

    let diameter = radius * 2.0;

    let mut sri = ShriYantra::new(
        radius,
        center,
        Some(diameter * 5.0 / 48.0),  // 1
        None,                         // 2
        Some(diameter * 26.5 / 48.0), // 3
        Some(diameter * 30.0 / 48.0), // 4
        Some(diameter * 42.0 / 48.0), // 5 , Tip of down triangle on the first base?
                                      //
    );

    let path_config_outer = Config::<T>::new(1.0, "blue".to_string(), "blue".to_string());
    let path_config_inner = Config::<T>::new(1.0, "red".to_string(), "red".to_string());
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
    // let canvas = canvas.add_circle(1.0, ut_1, point_config.clone());
    // let points = sri.get_all_points();
    let canvas = canvas
        .add_polygon(first_outer, path_config_outer.clone())
        .add_polygon(first_inner, path_config_inner.clone())
        .add_polygon(second_outer, path_config_outer.clone())
        .add_polygon(second_inner, path_config_inner.clone())
        .add_polygon(third_outer, path_config_outer.clone())
        .add_polygon(third_inner, path_config_inner.clone())
        .add_polygon(fourth_outer, path_config_outer.clone())
        .add_polygon(fourth_inner, path_config_inner.clone())
        .add_polygon(fifth_outer, path_config_outer.clone())
        .add_circle(1.0, center, config);

    svg::save("./unit_tests/yantra/add_sri.svg", &canvas).unwrap();
}
