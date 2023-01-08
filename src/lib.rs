use svg::Document;
pub type Canvas = Document;
pub mod art_forms;
pub mod canvas;
pub(crate) mod utils;

pub mod prelude {
    pub use super::art_forms::base_shapes::{BaseShapes, Config, SetConfig};
    pub use super::art_forms::leaves::{Leaves, LeafStyle, LeafStyleDetailed};
    pub use super::art_forms::yantra::{Yantra};
    pub use super::Canvas;
}
