extern crate typedgl_derive;

mod objects;
mod functions;

pub mod error;
pub mod load;

pub use load::load_with;

pub mod gl_fun{
    pub use super::functions::*;
    pub use super::functions::capabilties::*;
}
pub mod gl_obj{
    pub use super::objects::buffer::*;
    pub use super::objects::program::*;
    pub use super::objects::shader::*;
    pub use super::objects::vertex_array::*;
    pub use super::objects::uniform::*;
}