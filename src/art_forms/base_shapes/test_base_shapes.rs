use super::{Config, BaseShapes};

use geo::point;
use geo::polygon;
use svg::Document;

type Canvas = Document;
type T = f64;

#[test]
fn test_add_circle() {
    let view_size = 100.0;
    let canvas: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 20.0, y: 50.0);
    let radius = 50.0;
    let config = Config::<T>::new(3.0, "blue".to_string(), "yellow".to_string());
    let canvas = canvas.add_circle(radius, center, config);
    svg::save("./unit_tests/base_shapes/add_circle.svg", &canvas).unwrap();
}

#[test]
fn test_add_polygon() {
    let view_size = 100.0;
    let canvas: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let polygon = vec![
        point!(x: 20.0, y: 50.0),
        point!(x: 20.0, y: 0.0),
        point!(x: 0.0, y: 50.0),
    ];
    let config = Config::<T>::new(3.0, "blue".to_string(), "none".to_string());
    let canvas = canvas.add_polygon(polygon, config);
    svg::save("./unit_tests/base_shapes/add_polygon.svg", &canvas).unwrap();
}

#[test]
fn test_add_geo_polygon() {
    let view_size = 100.0;
    let canvas: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let polygon = polygon![
        (x: 20.0, y: 50.0),
        (x: 20.0, y: 0.0),
        (x: 0.0, y: 50.0),
    ];
    let config = Config::<T>::new(3.0, "blue".to_string(), "none".to_string());
    let canvas = canvas.add_geo_polygon(polygon, config);
    svg::save("./unit_tests/base_shapes/add_geo_polygon.svg", &canvas).unwrap();
}

#[test]
fn test_add_regular_n_gon() {
    let view_size = 100.0;
    let canvas: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 20.0, y: 50.0);
    let radius = 50.0;
    let config = Config::<T>::new(3.0, "blue".to_string(), "yellow".to_string());
    let canvas = canvas.add_regular_n_gon(radius, center, 30.00, 8, config);
    svg::save("./unit_tests/base_shapes/add_regular_n_gon.svg", &canvas).unwrap();
}

#[test]
fn test_add_star_polygon() {
    let view_size = 100.0;
    let canvas: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 50.0;
    let config = Config::<T>::new(1.5, "blue".to_string(), "none".to_string());
    let canvas = canvas.add_star_polygon(radius, center, 30.00, 13, 3, config);
    svg::save("./unit_tests/base_shapes/add_star_polygon.svg", &canvas).unwrap();
}

#[test]
fn test_add_isotoxal_star() {
    let view_size = 200.0;
    let canvas: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 50.0;
    let config = Config::<T>::new(1.5, "blue".to_string(), "yellow".to_string());
    let canvas = canvas
        .add_isotoxal_star(radius * 4., center, 0.00, 13, 3, config.clone(), Some(0))
        .add_isotoxal_star(radius * 2., center, 0.00, 13, 3, config.clone(), Some(13))
        .add_isotoxal_star(radius, center, 0.00, 13, 3, config, Some(10));
    svg::save("./unit_tests/base_shapes/add_isotoxal_star.svg", &canvas).unwrap();
}





