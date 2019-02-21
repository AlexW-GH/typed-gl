mod error;
mod objects;

pub mod prelude{
    pub use super::tgl_buffer;
    pub use super::tgl_program;
    pub use super::tgl_pipeline;
    pub use super::tgl_query;
    pub use super::tgl_renderbuffer;
    pub use super::tgl_sampler;
    pub use super::tgl_shader;
    pub use super::tgl_texture;
    pub use super::tgl_transform;
    pub use super::tgl_vertexarray;
}

pub mod tgl_buffer {
    pub use super::objects::buffer::{BufferTarget, BufferUsage, GLMutableBuffer, GLBuffer};
    pub use super::objects::buffer::func::{bind, unbind, buffer_data};
}

pub mod gl_framebuffer{

}

pub mod tgl_program{

}

pub mod tgl_pipeline {

}

pub mod tgl_query {

}

pub mod tgl_renderbuffer {

}

pub mod tgl_sampler {

}

pub mod tgl_shader {

}

pub mod tgl_texture {

}

pub mod tgl_transform {

}

pub mod tgl_vertexarray {
    pub use super::objects::vertex_array::{GLVertexArray};
    pub use super::objects::vertex_array::func::{bind, unbind};
}
