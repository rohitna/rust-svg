use crate::art_forms::yantra::{Config, LeafStyle, Yantra};
use geo::point;
use geo::polygon;
use svg::Document;

type Canvas = Document;
type T = f64;

#[test]
fn test_add_circle() {
    let view_size = 100.0;
    let yantra: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 20.0, y: 50.0);
    let radius = 50.0;
    let config = Config::<T>::new(3.0, "blue".to_string(), "yellow".to_string());
    let yantra = yantra.add_circle(radius, center, config);
    svg::save("./unit_tests/yantras/add_circle.svg", &yantra).unwrap();
}

#[test]
fn test_add_polygon() {
    let view_size = 100.0;
    let yantra: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let polygon = vec![
        point!(x: 20.0, y: 50.0),
        point!(x: 20.0, y: 0.0),
        point!(x: 0.0, y: 50.0),
    ];
    let config = Config::<T>::new(3.0, "blue".to_string(), "none".to_string());
    let yantra = yantra.add_polygon(polygon, config);
    svg::save("./unit_tests/yantras/add_polygon.svg", &yantra).unwrap();
}

#[test]
fn test_add_geo_polygon() {
    let view_size = 100.0;
    let yantra: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let polygon = polygon![
        (x: 20.0, y: 50.0),
        (x: 20.0, y: 0.0),
        (x: 0.0, y: 50.0),
    ];
    let config = Config::<T>::new(3.0, "blue".to_string(), "none".to_string());
    let yantra = yantra.add_geo_polygon(polygon, config);
    svg::save("./unit_tests/yantras/add_geo_polygon.svg", &yantra).unwrap();
}

#[test]
fn test_add_regular_n_gon() {
    let view_size = 100.0;
    let yantra: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 20.0, y: 50.0);
    let radius = 50.0;
    let config = Config::<T>::new(3.0, "blue".to_string(), "yellow".to_string());
    let yantra = yantra.add_regular_n_gon(radius, center, 30.00, 8, config);
    svg::save("./unit_tests/yantras/add_regular_n_gon.svg", &yantra).unwrap();
}

#[test]
fn test_add_star_polygon() {
    let view_size = 100.0;
    let yantra: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 50.0;
    let config = Config::<T>::new(1.5, "blue".to_string(), "none".to_string());
    let yantra = yantra.add_star_polygon(radius, center, 30.00, 13, 3, config);
    svg::save("./unit_tests/yantras/add_star_polygon.svg", &yantra).unwrap();
}

#[test]
fn test_add_isotoxal_star() {
    let view_size = 200.0;
    let yantra: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 50.0;
    let config = Config::<T>::new(1.5, "blue".to_string(), "yellow".to_string());
    let yantra = yantra
        .add_isotoxal_star(radius * 4., center, 0.00, 13, 3, config.clone(), Some(0))
        .add_isotoxal_star(radius * 2., center, 0.00, 13, 3, config.clone(), Some(13))
        .add_isotoxal_star(radius, center, 0.00, 13, 3, config, Some(10));
    svg::save("./unit_tests/yantras/add_isotoxal_star.svg", &yantra).unwrap();
}

#[test]
fn test_add_single_circular_leaf() {
    let view_size = 200.0;
    let yantra = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 100.0;
    let config = Config::<T>::new(3.0, "blue".to_string(), "yellow".to_string());
    // add a circle, a cordate leaf, and two reniform leaf to the yantra
    let cordate_style = LeafStyle::<T>::new_cordate(40.0, 0.5, 24.0, 26.0);
    let reniform_style_1 = LeafStyle::<T>::new_reniform(40.0, 0.5, 24.0, 20.0);
    let reniform_style_2 = LeafStyle::<T>::new_reniform(40.0, 0.5, 20.0, 0.0);
    let yantra = yantra
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
    svg::save("./unit_tests/yantras/add_single_circular_leaf.svg", &yantra).unwrap();
}

#[test]
fn test_add_single_linear_leaf() {
    let view_size = 200.0;
    let yantra = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 100.0;
    let config = Config::<T>::new(3.0, "blue".to_string(), "yellow".to_string());
    // add a linear reniform leaf, and a circle to the yantra
    let reniform_style = LeafStyle::<T>::new_reniform(50.0, 0.5, 12.0, 39.0);
    let yantra = yantra
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
    svg::save("./unit_tests/yantras/add_single_linear_leaf.svg", &yantra).unwrap();
}

#[test]
fn test_add_circular_leaves_on_an_arc() {
    let view_size = 200.0;
    let yantra = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 100.0;
    let config = Config::<T>::new(3.0, "blue".to_string(), "yellow".to_string());
    // add a circle, and 10 cordate leaves in 0 - 180 degrees
    let cordate_style = LeafStyle::<T>::new_cordate(40.0, 1.0, 24.0, 20.0);
    let yantra = yantra
        .add_circle(radius, center, config.clone())
        .add_circular_leaves_on_an_arc(radius, center, 0.0, 270.0, 10, cordate_style, config);

    // Save the image
    svg::save(
        "./unit_tests/yantras/add_circular_leaves_on_an_arc.svg",
        &yantra,
    )
    .unwrap();
}

#[test]
fn test_add_circular_leaves_on_a_circle() {
    let view_size = 200.0;
    let yantra = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 100.0;
    let config = Config::<T>::new(3.0, "blue".to_string(), "yellow".to_string());
    // add a circle, and 10 cordate leaves in 0 - 180 degrees
    let cordate_style = LeafStyle::<T>::new_cordate(40.0, 1.0, 62.0, 28.0);
    let yantra = yantra
        .add_circle(radius, center, config.clone())
        .add_circular_leaves(radius, center, 0.0, 10, -1.0, cordate_style, config);

    // Save the image
    svg::save("./unit_tests/yantras/add_circular_leaves.svg", &yantra).unwrap();
}

use crate::art_forms::yantra::sri_yantra_geometry::ShriYantra;

#[test]
fn test_sri_points() {
    let view_size = 100.0;
    let yantra: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 100.0;
    let config = Config::<T>::new(1.0, "yellow".to_string(), "yellow".to_string());
    let yantra = yantra.add_circle(radius, center, config);

    let diameter = radius * 2.0;
    let mut sri = ShriYantra::new(
        radius,
        center,
        Some(diameter * 6.0 / 48.0), // 1
        Some(diameter * 17.0 / 48.0), // 2
        Some(diameter * 27.0 / 48.0), // 3
        Some(diameter * 30.0 / 48.0), // 4
        Some(diameter * 42.0 / 48.0), // 5 , Tip of down triangle on the first) base?
    );

    let point_config = Config::<T>::new(1.0, "black".to_string(), "none".to_string());
    sri.construct_all_points();
    // let yantra = yantra.add_circle(1.0, ut_1, point_config.clone());
    let points = sri.get_all_points();
    let yantra = yantra.add_circles(0.5, points, point_config);

    svg::save("./unit_tests/yantras/add_sri_points.svg", &yantra).unwrap();

}

#[test]
fn test_sri_yantra() {
    let view_size = 100.0;
    let yantra: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 100.0;
    let config = Config::<T>::new(1.0, "yellow".to_string(), "yellow".to_string());
    let yantra = yantra.add_circle(radius, center, config.clone());

    let diameter = radius * 2.0;

    let mut sri = ShriYantra::new(
        radius,
        center,
       Some(diameter * 5.0 / 48.0), // 1
       None, // 2
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
    // let yantra = yantra.add_circle(1.0, ut_1, point_config.clone());
    // let points = sri.get_all_points();
    let yantra = yantra
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

    svg::save("./unit_tests/yantras/add_sri.svg", &yantra).unwrap();

}


