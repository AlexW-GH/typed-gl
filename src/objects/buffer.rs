use gl;
use gl::types::*;
use super::super::error::{GLError, GLErrorKind};
use std::marker::PhantomData;
use std::mem;
use std::os::raw;

pub mod func{
    use super::*;

    pub fn unbind(target: BufferTarget) {
        unsafe {
            gl::BindBuffer(target.value(), 0);
        }
    }

    pub unsafe fn buffer_data<T>(target: BufferTarget, usage: BufferUsage, data: &[T], _: &GLMutableBuffer<T>){
        let data_ptr = &data[0] as *const _ as *const raw::c_void;
        gl::BufferData(
            target.value(),
            mem::size_of_val(data) as GLsizeiptr,
            data_ptr,
            usage.value(),
        );
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BufferTarget{
    UnboundBuffer,
    ArrayBuffer,
    AtomicCounterBuffer,
    CopyReadBuffer,
    CopyWriteBuffer,
    DrawIndirectBuffer,
    DispatchIndirectBuffer,
    ElementArrayBuffer,
    PixelPackBuffer,
    PixelUnpackBuffer,
    QueryBuffer,
    ShaderStorageBuffer,
    TextureBuffer,
    TransformFeedbackBuffer,
    UniformBuffer,
}

impl BufferTarget{
    pub fn value(&self) -> u32{
        use self::BufferTarget::*;
        match self{
            UnboundBuffer => 0,
            ArrayBuffer => gl::ARRAY_BUFFER,
            AtomicCounterBuffer => gl::ATOMIC_COUNTER_BUFFER,
            CopyReadBuffer => gl::COPY_READ_BUFFER,
            CopyWriteBuffer => gl::COPY_WRITE_BUFFER,
            DrawIndirectBuffer => gl::DRAW_INDIRECT_BUFFER,
            DispatchIndirectBuffer => gl::DISPATCH_INDIRECT_BUFFER,
            ElementArrayBuffer => gl::ELEMENT_ARRAY_BUFFER,
            PixelPackBuffer => gl::PIXEL_PACK_BUFFER,
            PixelUnpackBuffer => gl::PIXEL_UNPACK_BUFFER,
            QueryBuffer => gl::QUERY_BUFFER,
            ShaderStorageBuffer => gl::SHADER_STORAGE_BUFFER,
            TextureBuffer => gl::TEXTURE_BUFFER,
            TransformFeedbackBuffer => gl::TRANSFORM_FEEDBACK_BUFFER,
            UniformBuffer => gl::UNIFORM_BUFFER,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BufferUsage{
    StaticDraw
}

impl BufferUsage{
    pub fn value(&self)  -> u32{
        use self::BufferUsage::*;
        match self{
            StaticDraw => gl::STATIC_DRAW
        }
    }
}

pub trait GLBuffer{
    fn bind(&self, target: BufferTarget);
}

#[derive(Debug)]
pub struct GLMutableBuffer<T> {
    name: GLuint,
    phantom_data: PhantomData<T>,
}

impl <T> GLMutableBuffer<T>{
    pub fn new() -> Result<GLMutableBuffer<T>, GLError> {
        let name = create_buffer()?;

        Ok(GLMutableBuffer{name, phantom_data: PhantomData})
    }
}

impl <T> GLBuffer for GLMutableBuffer<T>{
    fn bind(&self, target: BufferTarget) {
        unsafe {
            gl::BindBuffer(target.value(), self.name);
        }
    }
}

impl <T> Drop for GLMutableBuffer<T> {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.name); }
    }
}

fn create_buffer() -> Result<GLuint, GLError>{
    let mut name: GLuint = 0;
    unsafe {
        gl::GenBuffers(1, &mut name);
    }
    if name != 0 { Ok(name) } else { Err(GLErrorKind::BufferCreation)? }
}
