use crate::error::GLErrorKind;
use gl::types::*;
use crate::error::GLError;
use crate::objects::shader::GLShader;
use crate::gl_wrapper::GL;
use crate::objects::shader::ShaderType;
use std::ptr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GetProgramIvParam {
    DeleteStatus,
    LinkStatus,
    ValidateStatus,
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
            AttachedShaders => gl::ATTACHED_SHADERS,
            ActiveAttributes => gl::ACTIVE_ATTRIBUTES,
            ActiveAttributeMaxLength => gl:: ACTIVE_ATTRIBUTE_MAX_LENGTH,
            ActiveUniforms => gl::ACTIVE_UNIFORMS,
            ActiveUniformMaxLength => gl::ACTIVE_UNIFORM_MAX_LENGTH,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GetProgramIvResult {
    BooleanResult(bool),
    IntegerResult(i32),
}

#[derive(Debug)]
pub struct GLProgram{
    name: GLuint,
    shaders: Vec<ShaderNameTypePair>
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ShaderNameTypePair{
    shader_name: GLuint,
    shader_type: ShaderType
}

impl GLProgram{
    pub fn new() -> Result<Self, GLError> {
        let name;
        unsafe {
            name = GL::create_program();
        }
        if name == 0 {
            return Err(GLErrorKind::ProgramCreation)?;
        }
        Ok(GLProgram{name, shaders: Vec::new()})
    }

    pub fn attach_shader(&mut self, shader: &GLShader) -> Result<(), GLError>{
        let name_position = self.shaders.iter()
            .map(|pair| pair.shader_name)
            .position(|name| name == shader.name());
        let type_position = self.shaders.iter()
            .map(|pair| pair.shader_type)
            .position(|shader_type| shader_type == shader.shader_type());

        if name_position.is_some(){
            Err(GLErrorKind::ShaderAlreadyPresent)?
        } else if type_position.is_some(){
            Err(GLErrorKind::TypeAlreadyPresent)?
        } else {
            unsafe {
                GL::attach_shader(self.name, shader.name());
            }
            self.shaders.push(ShaderNameTypePair { shader_name: shader.name(), shader_type: shader.shader_type() });
        }
        Ok(())
    }

    pub fn detach_shader(&mut self, shader: &GLShader) -> Result<(), GLError>{
        match self.shaders.iter().position(|value| value.shader_name == shader.name()) {
            Some(index) => {
                unsafe{
                    GL::detach_shader(self.name, shader.name());
                }
                self.shaders.remove(index);
            }
            None => Err(GLErrorKind::ShaderNotPresent)?
        }
        Ok(())
    }


    pub fn link_program(&self){
        unsafe {
            GL::link_program(self.name)
        }
    }

    pub fn validate_program(&self){
        unsafe{
            GL::validate_program(self.name)
        }
    }

    pub fn use_program(&self){
        unsafe{
            GL::use_program(self.name);
        }
    }

    pub fn get_program_iv(&self, param: GetProgramIvParam) -> GetProgramIvResult{
        use self::GetProgramIvParam::*;
        let mut result = gl::FALSE as GLint;
        unsafe {
            GL::get_programiv(self.name, param.value(), &mut result);
        }
        match param {
            DeleteStatus => GetProgramIvResult::BooleanResult(result as u8 == gl::TRUE),
            LinkStatus => GetProgramIvResult::BooleanResult(result as u8 == gl::TRUE),
            ValidateStatus => GetProgramIvResult::BooleanResult(result as u8 == gl::TRUE),
            AttachedShaders => GetProgramIvResult::IntegerResult(result),
            ActiveAttributes => GetProgramIvResult::IntegerResult(result),
            ActiveAttributeMaxLength => GetProgramIvResult::IntegerResult(result),
            ActiveUniforms => GetProgramIvResult::IntegerResult(result),
            ActiveUniformMaxLength => GetProgramIvResult::IntegerResult(result),
        }
    }

    pub fn get_info_log(&self) -> Vec<u8> {
        let mut len = 0;
        unsafe{
            GL::get_programiv(self.name, gl::INFO_LOG_LENGTH, &mut len);

        }
        let mut buf = Vec::with_capacity(len as usize);
        unsafe{
            buf.set_len((len as usize) - 1);
            GL::get_program_info_log(
                self.name,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
        }
        buf
    }

    pub fn name(&self) -> u32 {
        self.name
    }
}

impl Drop for GLProgram {
    fn drop(&mut self) {
        unsafe { GL::delete_program(self.name) }
    }
}




#[cfg(test)]
mod tests{
    use speculate::speculate;

    use crate::objects::program::{GLProgram, GetProgramIvParam, GetProgramIvResult};
    use crate::objects::shader::GLShader;
    use crate::objects::shader::ShaderType;
    use crate::error::GLErrorKind;

    speculate! {
        describe "shaders" {

            before {
                let mut program = GLProgram::new().expect("Could not create GLProgram");
                let vs = GLShader::new(ShaderType::VertexShader).expect("Could not create GLShader");
            }

            it "can attach a shader" {
                program.attach_shader(&vs).expect("Could not attach Shader");

                assert_eq!(program.shaders.len(), 1);
            }

            it "can attach multiple different shaders" {
                let fs = GLShader::new(ShaderType::FragmentShader).expect("Could not create FragmentShader");

                program.attach_shader(&vs).expect("Could not attach first Shader");
                program.attach_shader(&fs).expect("Could not attach second Shader");

                assert_eq!(program.shaders.len(), 2);
            }

            it "can attach a shader of every type" {
                let tcs = GLShader::new(ShaderType::TessControlShader).expect("Could not create TessControlShader");
                let tes = GLShader::new(ShaderType::TessEvaluationShader).expect("Could not create TessEvaluationShader");
                let gs = GLShader::new(ShaderType::GeometryShader).expect("Could not create GeometryShader");
                let fs = GLShader::new(ShaderType::FragmentShader).expect("Could not create FragmentShader");
                let cs = GLShader::new(ShaderType::ComputeShader).expect("Could not create ComputeShader");

                program.attach_shader(&vs).expect("Could not attach first Shader");
                program.attach_shader(&tcs).expect("Could not attach TessControlShader");
                program.attach_shader(&tes).expect("Could not attach TessEvaluationShader");
                program.attach_shader(&gs).expect("Could not attach GeometryShader");
                program.attach_shader(&fs).expect("Could not attach FragmentShader");
                program.attach_shader(&cs).expect("Could not attach ComputeShader");

                assert_eq!(program.shaders.len(), 6);
            }

            it "can not attach the same shader twice" {
                program.attach_shader(&vs).expect("Could not attach first VertexShader");

                let error = program.attach_shader(&vs).expect_err("Expected failure to attach second VertexShader");

                assert_eq!(error.kind(), GLErrorKind::ShaderAlreadyPresent);
                assert_eq!(program.shaders.len(), 1);
            }

            it "can not attach the same type twice" {
                let vs2 = GLShader::new(ShaderType::VertexShader).expect("Could not create second VertexShader");
                program.attach_shader(&vs).expect("Could not attach first VertexShader");

                let error = program.attach_shader(&vs2).expect_err("Expected failure to attach second VertexShader");

                assert_eq!(error.kind(), GLErrorKind::TypeAlreadyPresent);
                assert_eq!(program.shaders.len(), 1);
            }

            it "can detach a shader" {
                program.attach_shader(&vs).expect("Could not attach Shader");

                program.detach_shader(&vs).expect("Could not detach Shader");

                assert_eq!(program.shaders.len(), 0);
            }

            it "only the detached shader is removed" {
                let fs = GLShader::new(ShaderType::FragmentShader).expect("Could not create FragmentShader");
                program.attach_shader(&vs).expect("Could not attach Shader");
                program.attach_shader(&fs).expect("Could not attach Shader");

                program.detach_shader(&vs).expect("Could not detach Shader");

                assert_eq!(program.shaders.len(), 1);
            }

            it "can detach all shaders" {
                let tcs = GLShader::new(ShaderType::TessControlShader).expect("Could not create TessControlShader");
                let tes = GLShader::new(ShaderType::TessEvaluationShader).expect("Could not create TessEvaluationShader");
                let gs = GLShader::new(ShaderType::GeometryShader).expect("Could not create GeometryShader");
                let fs = GLShader::new(ShaderType::FragmentShader).expect("Could not create FragmentShader");
                let cs = GLShader::new(ShaderType::ComputeShader).expect("Could not create ComputeShader");

                program.attach_shader(&vs).expect("Could not attach first Shader");
                program.attach_shader(&tcs).expect("Could not attach TessControlShader");
                program.attach_shader(&tes).expect("Could not attach TessEvaluationShader");
                program.attach_shader(&gs).expect("Could not attach GeometryShader");
                program.attach_shader(&fs).expect("Could not attach FragmentShader");
                program.attach_shader(&cs).expect("Could not attach ComputeShader");

                program.detach_shader(&vs).expect("Could not attach first Shader");
                program.detach_shader(&tcs).expect("Could not attach TessControlShader");
                program.detach_shader(&tes).expect("Could not attach TessEvaluationShader");
                program.detach_shader(&gs).expect("Could not attach GeometryShader");
                program.detach_shader(&fs).expect("Could not attach FragmentShader");
                program.detach_shader(&cs).expect("Could not attach ComputeShader");

                assert_eq!(program.shaders.len(), 0);
            }

            it "can not detach a shader on empty program" {
                let error = program.detach_shader(&vs).expect_err("Could not detach Shader");

                assert_eq!(error.kind(), GLErrorKind::ShaderNotPresent);
                assert_eq!(program.shaders.len(), 0);
            }

            it "can not detach a shader that is not present" {
                program.attach_shader(&vs).expect("Could not attach Shader");
                let vs2 = GLShader::new(ShaderType::VertexShader).expect("Could not create FragmentShader");
                let error = program.detach_shader(&vs2).expect_err("Could not detach Shader");

                assert_eq!(error.kind(), GLErrorKind::ShaderNotPresent);
                assert_eq!(program.shaders.len(), 1);
            }

            it "can not detach a shader with a different type that is not present" {
                program.attach_shader(&vs).expect("Could not attach Shader");
                let fs = GLShader::new(ShaderType::FragmentShader).expect("Could not create FragmentShader");
                let error = program.detach_shader(&fs).expect_err("Could not detach Shader");

                assert_eq!(error.kind(), GLErrorKind::ShaderNotPresent);
                assert_eq!(program.shaders.len(), 1);
            }

            it "can not detach a shader that was already detached" {
                program.attach_shader(&vs).expect("Could not attach Shader");
                program.detach_shader(&vs).expect("Could not detach Shader");

                let error = program.detach_shader(&vs).expect_err("Could not detach Shader");

                assert_eq!(error.kind(), GLErrorKind::ShaderNotPresent);
                assert_eq!(program.shaders.len(), 0);
            }
        }

         describe "programiv" {

            before {
                let mut program = GLProgram::new().expect("Could not create GLProgram");
            }

            it "can check delete status" {
                let result = program.get_program_iv(GetProgramIvParam::DeleteStatus);

                if let GetProgramIvResult::BooleanResult(status) = result {
                    assert_eq!(status, true);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check link status" {
                let result = program.get_program_iv(GetProgramIvParam::LinkStatus);

                if let GetProgramIvResult::BooleanResult(status) = result {
                    assert_eq!(status, true);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check validate status" {
                let result = program.get_program_iv(GetProgramIvParam::ValidateStatus);

                if let GetProgramIvResult::BooleanResult(status) = result {
                    assert_eq!(status, true);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check attached shaders" {
                let result = program.get_program_iv(GetProgramIvParam::AttachedShaders);

                if let GetProgramIvResult::IntegerResult(status) = result {
                    assert_eq!(status, 42);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check active attributes" {
                let result = program.get_program_iv(GetProgramIvParam::ActiveAttributes);

                if let GetProgramIvResult::IntegerResult(status) = result {
                    assert_eq!(status, 43);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check active attribute max length" {
                let result = program.get_program_iv(GetProgramIvParam::ActiveAttributeMaxLength);

                if let GetProgramIvResult::IntegerResult(status) = result {
                    assert_eq!(status, 44);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check active uniforms" {
                let result = program.get_program_iv(GetProgramIvParam::ActiveUniforms);

                if let GetProgramIvResult::IntegerResult(status) = result {
                    assert_eq!(status, 45);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check active uniform max length" {
                let result = program.get_program_iv(GetProgramIvParam::ActiveUniformMaxLength);

                if let GetProgramIvResult::IntegerResult(status) = result {
                    assert_eq!(status, 46);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can retrieve the info log" {
                let infoLog = program.get_info_log();
                assert_eq!(std::str::from_utf8(&infoLog).unwrap(), "success");
            }
         }
    }
}