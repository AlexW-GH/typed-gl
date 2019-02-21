use super::super::error::{GLError, GLErrorKind};
use gl::types::*;

pub mod func{
    use super::*;
    pub fn bind(vertex_array:  &GLVertexArray) {
        unsafe {
            gl::BindVertexArray(vertex_array.name());
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}


pub struct GLVertexArray{
    name: GLuint,
}

impl GLVertexArray{
    pub fn new() -> Result<Self, GLError> {
        let mut name: GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut name);
        }
        if name != 0 {
            Ok(GLVertexArray{name})
        } else {
            Err(GLErrorKind::VertexArrayCreation)?
        }
    }

    pub(super) fn name(&self) -> u32{
        self.name
    }
}

impl Drop for GLVertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.name); }
    }
}


