use gl::types::*;
pub struct GL{}

#[cfg(not(test))]
impl GL{
    #[inline(always)] pub unsafe fn create_program() -> GLuint{ gl::CreateProgram() }
    #[inline(always)] pub unsafe fn attach_shader(program: GLuint, shader_name: GLuint){ gl::AttachShader(program, shader_name) }
    #[inline(always)] pub unsafe fn detach_shader(program: GLuint, shader_name: GLuint){ gl::DetachShader(program, shader_name) }
    #[inline(always)] pub unsafe fn link_program(program: GLuint){ gl::LinkProgram(program) }
    #[inline(always)] pub unsafe fn validate_program(program: GLuint){ gl::ValidateProgram(program); }
    #[inline(always)] pub unsafe fn use_program(program: GLuint){ gl::UseProgram(program); }
    #[inline(always)] pub unsafe fn get_programiv(program: GLuint, param: GLuint, result: &mut GLint){ gl::GetProgramiv(program, param, result); }
    #[inline(always)] pub unsafe fn get_program_info_log(program: GLuint, max_length: GLint, length: *mut GLsizei, buf: *mut GLchar){ gl::GetProgramInfoLog(program, max_length, length, buf, ); }
    #[inline(always)] pub unsafe fn delete_program(program: GLuint){ gl::DeleteProgram(program) }
    #[inline(always)] pub unsafe fn create_shader(shader_type: u32) -> GLuint{ gl::CreateShader(shader_type) }
    #[inline(always)] pub unsafe fn shader_source(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint){ gl::ShaderSource(shader, count, string, length) }
    #[inline(always)] pub unsafe fn compile_shader(shader: GLuint){ gl::CompileShader(shader) }
    #[inline(always)] pub unsafe fn get_shaderiv(shader: GLuint, param: GLuint, result: &mut GLint){ gl::GetShaderiv(shader, param, result); }
    #[inline(always)] pub unsafe fn get_shader_info_log(shader: GLuint, max_length: GLint, length: *mut GLsizei, buf: *mut GLchar){ gl::GetShaderInfoLog(shader, max_length, length, buf); }
    #[inline(always)] pub unsafe fn get_shader_source(shader: GLuint, max_length: GLint, length: *mut GLsizei, buf: *mut GLchar){ gl::GetShaderSource(shader, max_length, length, buf); }
    #[inline(always)] pub unsafe fn delete_shader(name: GLuint){ gl::DeleteShader(name) }
}

#[cfg(test)] use std::sync::atomic::AtomicIsize;
#[cfg(test)] use std::sync::atomic::Ordering;

#[cfg(test)] static mut COUNTER: AtomicIsize = AtomicIsize::new(1);

#[cfg(test)]
impl GL{
    #[inline(always)] pub unsafe fn create_program() -> GLuint{
        COUNTER.fetch_add(1, Ordering::SeqCst) as GLuint
    }
    #[inline(always)] pub unsafe fn attach_shader(program_name: GLuint, shader_name: GLuint){}
    #[inline(always)] pub unsafe fn detach_shader(program_name: GLuint, shader_name: GLuint){}
    #[inline(always)] pub unsafe fn link_program(name: GLuint){}
    #[inline(always)] pub unsafe fn validate_program(name: GLuint){}
    #[inline(always)] pub unsafe fn use_program(name: GLuint){}
    #[inline(always)] pub unsafe fn get_programiv(name: GLuint, param: GLuint, result: &mut GLint){
        *result = match param{
            gl::DELETE_STATUS => 1,
            gl::LINK_STATUS => 1,
            gl::VALIDATE_STATUS => 1,
            gl::ATTACHED_SHADERS => 42,
            gl::ACTIVE_ATTRIBUTES => 43,
            gl::ACTIVE_ATTRIBUTE_MAX_LENGTH => 44,
            gl::ACTIVE_UNIFORMS => 45,
            gl::ACTIVE_UNIFORM_MAX_LENGTH => 46,
            gl::INFO_LOG_LENGTH => 8,
            _ => panic!("unknown status code")
        };
    }
    #[inline(always)] pub unsafe fn get_program_info_log(name: GLuint, max_length: GLint, length: *mut GLsizei, buf: *mut GLchar){
        let mut result = 0;
        Self::get_programiv(name, gl::INFO_LOG_LENGTH, &mut result);
        if result == max_length {
            let message = b"success";
            for (index, character) in message.iter().enumerate(){
                *buf.offset(index as isize) = *character as i8;
            }
        } else {
            let message = b"failure";
            for (index, character) in message.iter().enumerate(){
                *buf.offset(index as isize) = *character as i8;
            }
        }
    }
    #[inline(always)] pub unsafe fn delete_program(name: GLuint){}
    #[inline(always)] pub unsafe fn create_shader(shader_type: u32) -> GLuint{
        COUNTER.fetch_add(1, Ordering::SeqCst) as GLuint
    }
    #[inline(always)] pub unsafe fn shader_source(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint){}
    #[inline(always)] pub unsafe fn compile_shader(shader: GLuint){}
    #[inline(always)] pub unsafe fn get_shaderiv(shader: GLuint, param: GLuint, result: &mut GLint){}
    #[inline(always)] pub unsafe fn get_shader_info_log(shader: GLuint, max_length: GLint, length: *mut GLsizei, buf: *mut GLchar){}
    #[inline(always)] pub unsafe fn get_shader_source(shader: GLuint, max_length: GLint, length: *mut GLsizei, buf: *mut GLchar){}
    #[inline(always)] pub unsafe fn delete_shader(name: GLuint){}
}