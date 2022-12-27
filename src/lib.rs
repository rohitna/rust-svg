use svg::Document;
pub type Canvas = Document;
pub mod art_forms;
pub mod canvas;
pub(crate) mod utils;

pub mod prelude {
    pub use super::art_forms::yantra::{Config, LeafStyle, Yantra};
    pub use super::Canvas;
}
