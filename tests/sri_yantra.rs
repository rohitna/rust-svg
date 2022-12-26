use svg_art::prelude::*;
use geo::point;

#[test]
fn sri_yantra() {
    let view_size = 100.0;
    let yantra: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 20.0, y: 50.0);
    let radius = 50.0;
    let config = Config::new(3.0, "blue".to_string(), "yellow".to_string());
    let yantra = yantra.add_circle(radius, center, config);
    svg::save("./yantra_examples/sri_yantra.svg", &yantra).unwrap();
}
