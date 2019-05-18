use crate::error::GLError;
use crate::error::GLErrorKind;
use crate::objects::shader::{ShaderType, GLShader};
use gl::types::*;
use std::ptr;
use std::ffi::CString;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GetProgramIvParam {
    DeleteStatus,
    LinkStatus,
    ValidateStatus,
    AttachedShaders,
    ActiveAttributes,
    ActiveAttributeMaxLength,
    ActiveUniforms,
    ActiveUniformMaxLength,
}

impl GetProgramIvParam {
    pub fn value(self) -> u32 {
        use self::GetProgramIvParam::*;
        match self {
            DeleteStatus => gl::DELETE_STATUS,
            LinkStatus => gl::LINK_STATUS,
            ValidateStatus => gl::VALIDATE_STATUS,
            AttachedShaders => gl::ATTACHED_SHADERS,
            ActiveAttributes => gl::ACTIVE_ATTRIBUTES,
            ActiveAttributeMaxLength => gl::ACTIVE_ATTRIBUTE_MAX_LENGTH,
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

#[derive(Debug, Hash)]
pub struct GLProgram {
    name: GLuint,
    shaders: Vec<ShaderNameTypePair>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct ShaderNameTypePair {
    shader_name: GLuint,
    shader_type: ShaderType,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct GLUniformLocation<T>{
    pub(crate) program: GLuint,
    pub(crate) location: GLint,
    pd: PhantomData<T>,
}

impl GLProgram {
    pub fn new() -> Result<Self, GLError> {
        let name;
        unsafe {
            name = gl::CreateProgram();
        }
        if name == 0 {
            return Err(GLErrorKind::ProgramCreation)?;
        }
        Ok(GLProgram {
            name,
            shaders: Vec::new(),
        })
    }

    pub fn attach_shader(&mut self, shader: &GLShader) -> Result<(), GLError> {
        let name_position = self
            .shaders
            .iter()
            .map(|pair| pair.shader_name)
            .position(|name| name == shader.name());
        let type_position = self
            .shaders
            .iter()
            .map(|pair| pair.shader_type)
            .position(|shader_type| shader_type == shader.shader_type());

        if name_position.is_some() {
            Err(GLErrorKind::ShaderAlreadyPresent)?
        } else if type_position.is_some() {
            Err(GLErrorKind::TypeAlreadyPresent)?
        } else {
            unsafe {
                gl::AttachShader(self.name, shader.name());
            }
            self.shaders.push(ShaderNameTypePair {
                shader_name: shader.name(),
                shader_type: shader.shader_type(),
            });
        }
        Ok(())
    }

    pub fn detach_shader(&mut self, shader: &GLShader) -> Result<(), GLError> {
        match self
            .shaders
            .iter()
            .position(|value| value.shader_name == shader.name())
        {
            Some(index) => {
                unsafe {
                    gl::DetachShader(self.name, shader.name());
                }
                self.shaders.remove(index);
            }
            None => Err(GLErrorKind::ShaderNotPresent)?,
        }
        Ok(())
    }

    pub fn link_program(&self) {
        unsafe { gl::LinkProgram(self.name) }
    }

    pub fn validate_program(&self) {
        unsafe { gl::ValidateProgram(self.name) }
    }

    pub fn use_program(&self, is_used: bool) {
        unsafe {
            if is_used{
                gl::UseProgram(self.name);
            } else {
                gl::UseProgram(0);
            }
        }
    }

    pub fn get_program_iv(&self, param: GetProgramIvParam) -> GetProgramIvResult {
        use self::GetProgramIvParam::*;
        let mut result = i32::from(gl::FALSE);
        unsafe {
            gl::GetProgramiv(self.name, param.value(), &mut result);
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
        unsafe {
            gl::GetProgramiv(self.name, gl::INFO_LOG_LENGTH, &mut len);
        }
        let mut buf = Vec::with_capacity(len as usize);
        if len > 0 {
            unsafe {
                buf.set_len((len as usize) - 1);
                gl::GetProgramInfoLog(
                    self.name,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
            }
        }
        buf
    }

    pub fn get_uniform_location<T>(&self, uniform_name: &str) -> Result<GLUniformLocation<T>, GLError> {
        use failure::ResultExt;
        if uniform_name.starts_with("gl_"){
            return Err(GLErrorKind::ReservedUniformPrefix)?;
        }
        let location: i32;
        unsafe {
            location = gl::GetUniformLocation(self.name, CString::new(uniform_name.as_bytes()).context(GLErrorKind::InvalidUniformName)?.as_ptr());
        }
        if location == -1 {
            return Err(GLErrorKind::UniformNotPresent)?;
        }
        Ok(GLUniformLocation::<T>{program: self.name(), location, pd: PhantomData })
    }

    pub fn name(&self) -> u32 {
        self.name
    }
}

impl Drop for GLProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.name) }
    }
}

#[cfg(test)]
mod tests {
    use speculate::speculate;

    use crate::error::GLErrorKind;
    use crate::objects::program::{GLProgram, GetProgramIvParam, GetProgramIvResult};
    use crate::objects::shader::GLShader;
    use crate::objects::shader::ShaderType;

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
            }

        it "can create a program" {
            let mut program = GLProgram::new().expect("Could not create GLProgram");
            assert_ne!(program.name(), 0);
        }

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
                    assert_eq!(status, false);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check link status not linked" {
                let result = program.get_program_iv(GetProgramIvParam::LinkStatus);

                if let GetProgramIvResult::BooleanResult(status) = result {
                    assert_eq!(status, false);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check link status linked" {
                let shader = GLShader::new(ShaderType::VertexShader).expect("Could not create GLShader");
                let shader_source = b"
                    #version 330

                    out vec3 fragPos;

                    uniform int someValue;

                    void main(){
                        fragPos = (vec4(someValue, someValue, someValue, 1.0)).xyz;
                    }";
                shader.shader_source(shader_source);
                shader.compile_shader();
                program.attach_shader(&shader).expect("Could not attach shader");
                program.link_program();
                let result = program.get_program_iv(GetProgramIvParam::LinkStatus);

                if let GetProgramIvResult::BooleanResult(status) = result {
                    assert_eq!(status, true);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check link status linked error in shader" {
                let shader = GLShader::new(ShaderType::VertexShader).expect("Could not create GLShader");
                let shader_source = b"
                    #version 330

                    out vec3 fragPos;

                    uniform intxxx someValue;

                    void main(){
                        fragPos = (vec4(someValue, someValue, someValue, 1.0)).xyz;
                    }";
                shader.shader_source(shader_source);
                shader.compile_shader();
                program.attach_shader(&shader).expect("Could not attach shader");
                program.link_program();
                let result = program.get_program_iv(GetProgramIvParam::LinkStatus);

                if let GetProgramIvResult::BooleanResult(status) = result {
                    assert_eq!(status, false);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check validate status not initialized" {
                let shader = GLShader::new(ShaderType::VertexShader).expect("Could not create GLShader");
                program.validate_program();
                let result = program.get_program_iv(GetProgramIvParam::ValidateStatus);

                if let GetProgramIvResult::BooleanResult(status) = result {
                    assert_eq!(status, false);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check validate status successfully" {
                let shader = GLShader::new(ShaderType::VertexShader).expect("Could not create GLShader");
                let shader_source = b"
                    #version 330

                    out vec3 fragPos;

                    uniform int someValue;

                    void main(){
                        fragPos = (vec4(someValue, someValue, someValue, 1.0)).xyz;
                    }";
                shader.shader_source(shader_source);
                shader.compile_shader();
                program.attach_shader(&shader).expect("Could not attach shader");
                program.link_program();
                program.validate_program();
                let result = program.get_program_iv(GetProgramIvParam::ValidateStatus);

                if let GetProgramIvResult::BooleanResult(status) = result {
                    assert_eq!(status, true);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check validate status error in shader" {
                let shader = GLShader::new(ShaderType::VertexShader).expect("Could not create GLShader");
                let shader_source = b"
                    #version 330

                    out vec3 fragPos;

                    uniform intxxx someValue;

                    void main(){
                        fragPos = (vec4(someValue, someValue, someValue, 1.0)).xyz;
                    }";
                shader.shader_source(shader_source);
                shader.compile_shader();
                program.attach_shader(&shader).expect("Could not attach shader");
                program.link_program();
                program.validate_program();
                let result = program.get_program_iv(GetProgramIvParam::ValidateStatus);

                if let GetProgramIvResult::BooleanResult(status) = result {
                    assert_eq!(status, false);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check attached shaders" {
            let shader = GLShader::new(ShaderType::VertexShader).expect("Could not create GLShader");
                let shader_source = b"
                    #version 330

                    out vec3 fragPos;

                    uniform intxxx someValue;

                    void main(){
                        fragPos = (vec4(someValue, someValue, someValue, 1.0)).xyz;
                    }";
                shader.shader_source(shader_source);
                shader.compile_shader();
                program.attach_shader(&shader).expect("Could not attach shader");
                program.link_program();
                program.validate_program();

                let result = program.get_program_iv(GetProgramIvParam::AttachedShaders);

                if let GetProgramIvResult::IntegerResult(status) = result {
                    assert_eq!(status, 1);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check attached shaders with no attached shaders" {
                let result = program.get_program_iv(GetProgramIvParam::AttachedShaders);

                if let GetProgramIvResult::IntegerResult(status) = result {
                    assert_eq!(status, 0);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check active uniforms" {
                let shader = GLShader::new(ShaderType::VertexShader).expect("Could not create GLShader");
                let shader_source = b"
                    #version 330

                    out vec3 fragPos;

                    uniform int someValue;

                    void main(){
                        fragPos = (vec4(someValue, someValue, someValue, 1.0)).xyz;
                    }";
                shader.shader_source(shader_source);
                shader.compile_shader();
                program.attach_shader(&shader).expect("Could not attach shader");
                program.link_program();
                let uniform_location = program.get_uniform_location::<i32>("someValue").expect("could not retrieve uniform location");

                let result = program.get_program_iv(GetProgramIvParam::ActiveUniforms);

                if let GetProgramIvResult::IntegerResult(status) = result {
                    assert_eq!(status, 1);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can check active uniform max length" {
                let shader = GLShader::new(ShaderType::VertexShader).expect("Could not create GLShader");
                let shader_source = b"
                    #version 330

                    out vec3 fragPos;

                    uniform int someValue;

                    void main(){
                        fragPos = (vec4(someValue, someValue, someValue, 1.0)).xyz;
                    }";
                shader.shader_source(shader_source);
                shader.compile_shader();
                program.attach_shader(&shader).expect("Could not attach shader");
                program.link_program();

                let result = program.get_program_iv(GetProgramIvParam::ActiveUniformMaxLength);

                if let GetProgramIvResult::IntegerResult(status) = result {
                    assert_eq!(status, 10);
                } else {
                    panic!("Wrong result type");
                }
            }

            it "can retrieve the info log with link error" {
                let shader = GLShader::new(ShaderType::VertexShader).expect("Could not create GLShader");
                let shader_source = b"
                    #version 330

                    out vec3 fragPos;

                    uniform intx someValue;

                    void main(){
                        fragPos = (vec4(someValue, someValue, someValue, 1.0)).xyz;
                    }";
                shader.shader_source(shader_source);
                shader.compile_shader();
                program.attach_shader(&shader).expect("Could not attach shader");
                program.link_program();

                let infoLog = program.get_info_log();
                assert_eq!(std::str::from_utf8(&infoLog).unwrap(), "Vertex info\n-----------\n0(6) : \
                error C0000: syntax error, unexpected identifier, expecting \
                \'{\' at token \"someValue\"\n(0) : error C2003: incompatible options for link\n");
            }

            it "can retrieve the info log with no error" {
                let shader = GLShader::new(ShaderType::VertexShader).expect("Could not create GLShader");
                let shader_source = b"
                    #version 330

                    out vec3 fragPos;

                    uniform int someValue;

                    void main(){
                        fragPos = (vec4(someValue, someValue, someValue, 1.0)).xyz;
                    }";
                shader.shader_source(shader_source);
                shader.compile_shader();
                program.attach_shader(&shader).expect("Could not attach shader");
                program.link_program();

                let infoLog = program.get_info_log();
                assert_eq!(std::str::from_utf8(&infoLog).unwrap(), "");
            }
         }

         describe "uniform" {

            before {
                let mut program = GLProgram::new().expect("Could not create GLProgram");
                let shader = GLShader::new(ShaderType::VertexShader).expect("Could not create GLShader");
                let shader_source = b"
                    #version 330

                    out vec3 fragPos;

                    uniform int someValue;

                    void main(){
                        fragPos = (vec4(someValue, someValue, someValue, 1.0)).xyz;
                    }";
                shader.shader_source(shader_source);
                shader.compile_shader();
                program.attach_shader(&shader).expect("Could not attach shader");
                program.link_program();
            }

            it "can retrieve shader source" {
                let result = shader.get_shader_source();
                assert_eq!(result, shader_source.to_vec())
            }

            it "can retrieve a uniform location" {

                let uniform_location = program.get_uniform_location::<i32>("someValue").expect("could not retrieve uniform location");

                use crate::objects::uniform::Uniform;

                assert!(uniform_location.location >= 0);
            }

            it "returns error on gl_ - prefixed name" {
                let error = program.get_uniform_location::<f32>("gl_uniform").expect_err("Expected failure to get uniform location");

                assert_eq!(error.kind(), GLErrorKind::ReservedUniformPrefix);
            }

            it "returns error when uniform not found" {
                let error = program.get_uniform_location::<f32>("invalid").expect_err("Expected failure to get uniform location");

                assert_eq!(error.kind(), GLErrorKind::UniformNotPresent);
            }
         }
    }
}
