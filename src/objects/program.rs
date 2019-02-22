use crate::error::GLErrorKind;
use gl;
use gl::types::*;
use crate::error::GLError;
use crate::objects::shader::GLShader;

#[derive(Clone, Copy, Debug)]
pub enum GetProgramIvParam {
    DeleteStatus,
    LinkStatus,
    ValidateStatus,
    InfoLogLength,
    AttachedShaders,
    ActiveAttributes,
    ActiveAttributeMaxLength,
    ActiveUniforms,
    ActiveUniformMaxLength
}

impl GetProgramIvParam {
    pub fn value(&self) -> u32 {
        use self::GetProgramIvParam::*;
        match self {
            DeleteStatus => gl::DELETE_STATUS,
            LinkStatus => gl::LINK_STATUS,
            ValidateStatus => gl::VALIDATE_STATUS,
            InfoLogLength => gl::INFO_LOG_LENGTH,
            AttachedShaders => gl::ATTACHED_SHADERS,
            ActiveAttributes => gl::ACTIVE_ATTRIBUTES,
            ActiveAttributeMaxLength => gl:: ACTIVE_ATTRIBUTE_MAX_LENGTH,
            ActiveUniforms => gl::ACTIVE_UNIFORMS,
            ActiveUniformMaxLength => gl::ACTIVE_UNIFORM_MAX_LENGTH,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GetProgramIvResult {
    BooleanResult(bool),
    IntegerResult(i32),
}

#[derive(Debug)]
pub struct GLProgram{
    name: GLuint,
}

impl GLProgram{
    pub fn new() -> Result<Self, GLError> {
        let name;
        unsafe {
            name = gl::CreateProgram();
        }
        if name == 0 {
            return Err(GLErrorKind::ProgramCreation)?;
        }
        Ok(GLProgram{name})
    }

    pub fn attach_shader(&self, shader: &GLShader){
        unsafe{
            gl::AttachShader(self.name, shader.name());
        }
    }

    pub fn link_program(&self){
        unsafe {
            gl::LinkProgram(self.name)
        }
    }

    pub fn validate_program(&self){
        unsafe{
            gl::ValidateProgram(self.name);
        }
    }

    pub fn get_program_iv(&self, param: GetProgramIvParam) -> GetProgramIvResult{
        use self::GetProgramIvParam::*;
        let mut result = gl::FALSE as GLint;
        unsafe {
            gl::GetProgramiv(self.name, param.value(), &mut result);
        }
        match param {
            DeleteStatus => GetProgramIvResult::BooleanResult(result as u8 == gl::TRUE),
            LinkStatus => GetProgramIvResult::BooleanResult(result as u8 == gl::TRUE),
            ValidateStatus => GetProgramIvResult::BooleanResult(result as u8 == gl::TRUE),
            InfoLogLength => GetProgramIvResult::IntegerResult(result),
            AttachedShaders => GetProgramIvResult::IntegerResult(result),
            ActiveAttributes => GetProgramIvResult::IntegerResult(result),
            ActiveAttributeMaxLength => GetProgramIvResult::IntegerResult(result),
            ActiveUniforms => GetProgramIvResult::IntegerResult(result),
            ActiveUniformMaxLength => GetProgramIvResult::IntegerResult(result),
        }
    }
}