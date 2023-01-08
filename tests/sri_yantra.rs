use svg_art::prelude::*;
use geo::{point, CoordFloat};

#[test]
fn sri_yantra() {
    let view_size = 150.0;
    let canvas: Canvas = Canvas::new().set(
        "viewBox",
        (-view_size, -view_size, 2.0 * view_size, 2.0 * view_size),
    );
    let center = point!(x: 0.0, y: 0.0);
    let radius = 100.0;
    let config = Config::new(1.0, "none".to_string(), "none".to_string());
    let background_color = "#E8FB62";
    // let smallest_circle_config = new_config(config, background_color);
    // let bindu_config = Config::new(1.0, "none".to_string(), background_color.to_string());


    let sri_configs = [
        new_config(config.clone(), "#7b7064"), // glow worm color
        new_config(config.clone(), background_color),
        new_config(config.clone(), "#fd5e36"), // Japakusuma flower color
        new_config(config.clone(), background_color),
        new_config(config.clone(), "#7c96f7"), // some blue
        new_config(config.clone(), background_color),
        new_config(config.clone(), "#fd9d97"), // dadini flower color
        new_config(config.clone(), background_color),
        new_config(config.clone(), "white")
    ];

    let cordate_style = LeafStyle::new_cordate(15.0, 1.02, 12.0, 13.0);
    let reniform_style = LeafStyle::new_reniform(25.0, 1.1, 12.0, 13.0);
    let yantra = canvas
    .add_circle(radius, center, new_config(config.clone(), background_color))
    .add_circular_leaves(radius, center, -360.0 / 32.0, 16, 1.0, reniform_style, new_config(config.clone(), "pink"))
    .add_circle(radius, center, new_config(config.clone(), background_color))
    .add_circular_leaves(radius, center, -360.0 / 16.0, 8, 1.0, cordate_style, new_config(config.clone(), "#900007")) // Bandhuka flower color
    .add_sri(radius, center, sri_configs)
    .add_circle(1.0, center, new_config(config.clone(), "#f33625")); //kumkum color
    svg::save("./examples/sri_yantra.svg", &yantra).unwrap();
}

fn new_config<T: CoordFloat>(mut config: Config<T>, new_fill_color: &str) -> Config<T> {
    config.fill_color = new_fill_color.to_string();
    config
}
