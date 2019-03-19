use crate::error::GLError;
use crate::error::GLErrorKind;
use crate::objects::shader::ShaderType::ComputeShader;
use crate::objects::shader::ShaderType::FragmentShader;
use crate::objects::shader::ShaderType::GeometryShader;
use crate::objects::shader::ShaderType::TessControlShader;
use crate::objects::shader::ShaderType::TessEvaluationShader;
use crate::objects::shader::ShaderType::VertexShader;
use failure::ResultExt;
use gl::types::*;
use std::ffi::CString;
use std::ptr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ShaderType {
    VertexShader,
    TessControlShader,
    TessEvaluationShader,
    GeometryShader,
    FragmentShader,
    ComputeShader,
}

impl ShaderType {
    pub fn value(self) -> u32 {
        use self::ShaderType::*;
        match self {
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
            _ => Err(GLErrorKind::UnknownShader)?,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GetShaderIvParam {
    ShaderType,
    DeleteStatus,
    CompileStatus,
}

impl GetShaderIvParam {
    pub fn value(self) -> u32 {
        use self::GetShaderIvParam::*;
        match self {
            ShaderType => gl::SHADER_TYPE,
            DeleteStatus => gl::DELETE_STATUS,
            CompileStatus => gl::COMPILE_STATUS,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GetShaderIvResult {
    BooleanResult(bool),
    IntegerResult(i32),
    ShaderResult(ShaderType),
}

#[derive(Debug)]
pub struct GLShader {
    name: GLuint,
    shader_type: ShaderType,
}

impl GLShader {
    pub fn new(shader_type: ShaderType) -> Result<Self, GLError> {
        let name;
        unsafe {
            name = gl::CreateShader(shader_type.value());
        }
        if name == 0 {
            return Err(GLErrorKind::ShaderCreation)?;
        }
        Ok(GLShader { name, shader_type })
    }

    pub fn name(&self) -> u32 {
        self.name
    }

    pub fn shader_type(&self) -> ShaderType {
        self.shader_type
    }

    pub fn shader_source(&self, shader_src: &[u8]) -> Result<(), GLError> {
        let c_str = CString::new(shader_src).context(GLErrorKind::ShaderSourceInternalNull)?;
        let length = vec![shader_src.len() as GLint];
        unsafe {
            gl::ShaderSource(self.name, 1, &c_str.as_ptr(), length.as_ptr());
        }
        Ok(())
    }

    pub fn compile_shader(&self) {
        unsafe {
            gl::CompileShader(self.name);
        }
    }

    pub fn get_shader_iv(&self, param: GetShaderIvParam) -> Result<GetShaderIvResult, GLError> {
        let mut result = i32::from(gl::FALSE);
        unsafe {
            gl::GetShaderiv(self.name, param.value(), &mut result);
        }
        match param {
            GetShaderIvParam::ShaderType => {
                Ok(GetShaderIvResult::ShaderResult(ShaderType::from(result)?))
            }
            GetShaderIvParam::DeleteStatus => Ok(GetShaderIvResult::BooleanResult(
                result == i32::from(gl::TRUE),
            )),
            GetShaderIvParam::CompileStatus => Ok(GetShaderIvResult::BooleanResult(
                result == i32::from(gl::TRUE),
            )),
        }
    }

    pub fn get_info_log(&self) -> Vec<u8> {
        let mut len = 0;
        unsafe {
            gl::GetShaderiv(self.name, gl::INFO_LOG_LENGTH, &mut len);
        }
        let mut buf = Vec::with_capacity(len as usize);
        if len > 0 {
            unsafe {
                buf.set_len((len as usize) - 1);
                gl::GetShaderInfoLog(
                    self.name,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
            }
        }
        buf
    }

    pub fn get_shader_source(&self) -> Vec<u8> {
        let mut len = 0;
        unsafe {
            gl::GetShaderiv(self.name, gl::SHADER_SOURCE_LENGTH, &mut len);
        }
        let mut buf = Vec::with_capacity(len as usize);
        if len > 0 {
            unsafe {
                buf.set_len((len as usize) - 1);
                gl::GetShaderSource(
                    self.name,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
            }
        }
        buf
    }
}

impl Drop for GLShader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.name) }
    }
}

#[cfg(test)]
mod tests {
    use speculate::speculate;

    use crate::error::GLErrorKind;
    use crate::objects::shader::{GLShader, GetShaderIvParam, GetShaderIvResult, ShaderType};
    use crate::objects::program::GLProgram;

    speculate! {
        before {
                use glutin::{EventsLoop, WindowBuilder, ContextBuilder, WindowedContext};
                use glutin::dpi::PhysicalSize;
                use glutin::ContextTrait;

                let events_loop = EventsLoop::new();
                let context = ContextBuilder::new()
                    .with_vsync(false)
                    .build_headless(&events_loop, PhysicalSize{width: 800f64, height: 600f64})
                    .unwrap();
                unsafe{
                    context.make_current().unwrap();
                    gl::load_with(|symbol| context.get_proc_address(symbol) as *const _);
                }

                let mut program = GLProgram::new().expect("Could not create GLProgram");
                let vs = GLShader::new(ShaderType::VertexShader).expect("Could not create GLShader");
                let shader_source = b"
                    #version 330

                    out vec3 fragPos;

                    uniform float value;

                    void main(){
                        fragPos = (vec4(value, value, value, 1.0)).xyz;
                    }";
            }

        describe "creation" {

            it "can set the shader source" {
                let source = b"void main {}";

                vs.shader_source(source).expect("Could not set shader source");
            }

            it "can not set shader source with internal null" {
                let source = b"void main {\0}";

                let error = vs.shader_source(source).expect_err("Expected error while setting shader source");
                assert_eq!(error.kind(), GLErrorKind::ShaderSourceInternalNull);
            }
        }

        describe "shaderiv" {


            it "can check shader type" {
                let result = vs.get_shader_iv(GetShaderIvParam::ShaderType).unwrap();

                if let GetShaderIvResult::ShaderResult(shader_type) = result {
                    assert_eq!(shader_type, ShaderType::VertexShader);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check delete status" {
                let result = vs.get_shader_iv(GetShaderIvParam::DeleteStatus).unwrap();

                if let GetShaderIvResult::BooleanResult(status) = result {
                    assert_eq!(status, false);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check compile status for a compiled shader" {
                vs.shader_source(shader_source);
                vs.compile_shader();
                let result = vs.get_shader_iv(GetShaderIvParam::CompileStatus).unwrap();

                if let GetShaderIvResult::BooleanResult(status) = result {
                    assert_eq!(status, true);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check compile status for an uncompiled shader" {
                vs.shader_source(shader_source);
                let result = vs.get_shader_iv(GetShaderIvParam::CompileStatus).unwrap();

                if let GetShaderIvResult::BooleanResult(status) = result {
                    assert_eq!(status, false);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can retrieve the info log" {
                vs.shader_source(b"void main {}");
                vs.compile_shader();

                let infoLog = vs.get_info_log();
                assert_eq!(std::str::from_utf8(&infoLog).unwrap(), "0(1) : error C1014: \"main\" is not a function\n");
            }

            it "can retrieve shader source" {
                vs.shader_source(shader_source);
                let result = vs.get_shader_source();
                assert_eq!(result, shader_source.to_vec())
            }
         }
    }
}
