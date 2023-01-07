pub mod base_shapes_trait;
pub mod base_shapes_impl;
pub use base_shapes_trait::{BaseShapes, Config, SetConfig};

#[cfg(test)]
pub mod test_base_shapes;