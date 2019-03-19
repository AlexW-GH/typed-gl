use super::super::error::{GLError, GLErrorKind};
use gl::types::*;

pub use typedgl_derive::IsVertex;


#[derive(Debug)]
pub struct GLVertexArray {
    name: GLuint,
}

pub trait IsVertex {
    fn element_size(&self, index: usize) -> gl::types::GLint;
    fn element_type(&self, index: usize) -> VertexElementType;
    fn element_stride(&self) -> gl::types::GLsizei;
    fn element_pointer(&self, index: usize) -> *const std::os::raw::c_void;
    fn field_position(&self, field_name: &str) -> usize;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum VertexElementType{
    UnsignedByte,
    Byte,
    UnsignedShort,
    Short,
    Float,
}

pub struct VertexAttribPointer{
    index: u32
}

impl VertexAttribPointer {
    pub fn enable(&self){
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }
}

impl VertexElementType{
    pub fn value(&self) -> gl::types::GLenum {
        use VertexElementType::*;
        match self {
            UnsignedByte => gl::UNSIGNED_BYTE,
            Byte => gl::BYTE,
            UnsignedShort => gl::UNSIGNED_SHORT,
            Short => gl::SHORT,
            Float => gl::FLOAT,
        }
    }
}


impl GLVertexArray {
    pub fn new() -> Result<Self, GLError> {
        let mut name: GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut name);
        }
        if name != 0 {
            Ok(GLVertexArray { name })
        } else {
            Err(GLErrorKind::VertexArrayCreation)?
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.name);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn create_vertex_attrib_pointer(&self, vertices: &[impl IsVertex], field_name: &str, index: u32) -> Result<VertexAttribPointer, GLError>{
        let vertex = match vertices.get(0){
            Some(vertex) => vertex,
            None => Err(GLErrorKind::VertexArrayEmpty)?
        };
        if index >= gl::MAX_VERTEX_ATTRIBS{
            Err(GLErrorKind::TooManyVertexAttribs)?
        }

        let field_index = vertex.field_position(field_name);
        let field_size = vertex.element_size(field_index);
        let field_type =vertex.element_type(field_index).value();
        let stride = vertex.element_stride();
        let field_ptr = vertex.element_pointer(field_index);
        if field_size < 0 || field_size > 4{
            Err(GLErrorKind::WrongArraySize)?
        }
        unsafe {
            gl::VertexAttribPointer(
                index,
                field_size,
                field_type,
                gl::FALSE as GLboolean,
                stride,
                field_ptr
            );
        }
        Ok(VertexAttribPointer{index})
    }
}

impl Drop for GLVertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.name);
        }
    }
}
