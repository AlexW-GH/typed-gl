use super::super::error::{GLError, GLErrorKind};
use gl::types::*;

pub mod func{
    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

#[derive(Debug)]
pub struct GLVertexArray{
    name: GLuint,
}

impl GLVertexArray{
    pub fn new() -> Result<Self, GLError> {
        let mut name: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut name);
        }
        if name != 0 {
            Ok(GLVertexArray{name})
        } else {
            Err(GLErrorKind::VertexArrayCreation)?
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.name);
        }
    }
}

impl Drop for GLVertexArray {
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.name); }
    }
}


