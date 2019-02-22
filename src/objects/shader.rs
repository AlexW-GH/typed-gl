use crate::error::GLErrorKind;
use gl;
use gl::types::*;
use crate::error::GLError;
use std::ffi::CString;
use crate::objects::shader::ShaderType::VertexShader;
use crate::objects::shader::ShaderType::TessControlShader;
use crate::objects::shader::ShaderType::TessEvaluationShader;
use crate::objects::shader::ShaderType::GeometryShader;
use crate::objects::shader::ShaderType::FragmentShader;
use crate::objects::shader::ShaderType::ComputeShader;
use std::ptr;
use failure::ResultExt;

#[derive(Clone, Copy, Debug)]
pub enum ShaderType{
    VertexShader,
    TessControlShader,
    TessEvaluationShader,
    GeometryShader,
    FragmentShader,
    ComputeShader,
}

impl ShaderType{
    pub fn value(&self) -> u32{
        use self::ShaderType::*;
        match self{
            VertexShader => gl::VERTEX_SHADER,
            TessControlShader => gl::TESS_CONTROL_SHADER,
            TessEvaluationShader => gl::TESS_EVALUATION_SHADER,
            GeometryShader => gl::GEOMETRY_SHADER,
            FragmentShader => gl::FRAGMENT_SHADER,
            ComputeShader => gl::COMPUTE_SHADER,
        }
    }

    pub fn from(value: GLint) -> Result<ShaderType, GLError> {
        match value as u32 {
            gl::VERTEX_SHADER => Ok(VertexShader),
            gl::TESS_CONTROL_SHADER => Ok(TessControlShader),
            gl::TESS_EVALUATION_SHADER => Ok(TessEvaluationShader),
            gl::GEOMETRY_SHADER => Ok(GeometryShader),
            gl::FRAGMENT_SHADER => Ok(FragmentShader),
            gl::COMPUTE_SHADER => Ok(ComputeShader),
            _ => Err(GLErrorKind::UnknownShader)?
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GetShaderIvParam {
    ShaderType,
    DeleteStatus,
    CompileStatus,
}

impl GetShaderIvParam {
    pub fn value(&self) -> u32{
        use self::GetShaderIvParam::*;
        match self{
            ShaderType => gl::SHADER_TYPE,
            DeleteStatus => gl::DELETE_STATUS,
            CompileStatus => gl::COMPILE_STATUS
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GetShaderIvResult {
    BooleanResult(bool),
    IntegerResult(i32),
    ShaderResult(ShaderType),
}

#[derive(Debug)]
pub struct GLShader{
    name: GLuint,
}

impl GLShader{
    pub fn new(shader_type: ShaderType) -> Result<Self, GLError> {
        let name;
        unsafe {
            name = gl::CreateShader(shader_type.value());
        }
        if name == 0 {
            return Err(GLErrorKind::ShaderCreation)?;
        }
        Ok(GLShader{name})
    }

    pub fn name(&self) -> u32 {
        self.name
    }

    pub fn shader_source(&self, shader_src: &[u8]) -> Result<(), GLError>{
        let c_str = CString::new(shader_src)
            .context(GLErrorKind::ShaderSourceInternalNull)?;
        let length = vec![shader_src.len() as GLint];
        unsafe{
            gl::ShaderSource(self.name, 1, &c_str.as_ptr(), length.as_ptr());
        }
        Ok(())
    }

    pub fn compile_shader(&self){
        unsafe{
            gl::CompileShader(self.name);
        }
    }

    pub fn get_shader_iv(&self, param: GetShaderIvParam) -> Result<GetShaderIvResult, GLError>{
        let mut result = gl::FALSE as GLint;
        unsafe {
            gl::GetShaderiv(self.name, param.value(), &mut result);
        }
        match param {
            GetShaderIvParam::ShaderType => Ok(GetShaderIvResult::ShaderResult(ShaderType::from(result)?)),
            GetShaderIvParam::DeleteStatus => Ok(GetShaderIvResult::BooleanResult(result == gl::TRUE as GLint)),
            GetShaderIvParam::CompileStatus => Ok(GetShaderIvResult::BooleanResult(result == gl::TRUE as GLint)),
        }
    }

    pub fn get_shader_info_log(&self) -> Vec<u8> {
        let mut len = 0;
        unsafe{
            gl::GetShaderiv(self.name, gl::INFO_LOG_LENGTH, &mut len);

        }
        let mut buf = Vec::with_capacity(len as usize);
        unsafe{
            buf.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(
                self.name,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
        }
        buf
    }

    pub fn get_shader_source(&self) -> Vec<u8> {
        let mut len = 0;
        unsafe{
            gl::GetShaderiv(self.name, gl::SHADER_SOURCE_LENGTH, &mut len);

        }
        let mut buf = Vec::with_capacity(len as usize);
        unsafe{
            buf.set_len((len as usize) - 1);
            gl::GetShaderSource(
                self.name,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
        }
        buf
    }
}