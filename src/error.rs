use failure::Backtrace;
use failure::Context;
use failure::Fail;
use std::fmt;

#[derive(Debug)]
pub struct GLError {
    inner: Context<GLErrorKind>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum GLErrorKind {
    #[fail(display = "Program Creation Failed")]
    ProgramCreation,

    #[fail(display = "Shader Creation Failed")]
    ShaderCreation,

    #[fail(display = "Buffer Creation Failed")]
    BufferCreation,

    #[fail(display = "VertexArray Creation Failed")]
    VertexArrayCreation,

    #[fail(display = "Tried to reference a invalid or unknown shader")]
    UnknownShader,

    #[fail(display = "Internal null found in Shader source")]
    ShaderSourceInternalNull,

    #[fail(display = "Could not attach Shader, Shader is already present")]
    ShaderAlreadyPresent,

    #[fail(display = "Could not attach Shader, ShaderType is already present")]
    TypeAlreadyPresent,

    #[fail(display = "Could not detach Shader, not present")]
    ShaderNotPresent,

    #[fail(display = "Too Many Buffers have been requested")]
    TooManyBuffers,

    #[fail(display = "Could not get capabilites")]
    GetCapability,

    #[fail(display = "Supplied index is greater than or equal to GL_MAX_VERTEX_ATTRIBS.")]
    TooManyVertexAttribs,

    #[fail(display = "Array size of fields may only be 1, 2, 3 or 4")]
    WrongArraySize,

    #[fail(display = "Vertex array may not be empty")]
    VertexArrayEmpty,

    #[fail(display = "Name not an active uniform value, is associated with an atomic counter or is a named uniform block")]
    UniformNotPresent,

    #[fail(display = "Uniform is not allowed to start with the prefix 'gl_'")]
    ReservedUniformPrefix,

    #[fail(display = "Uniform name needs to be null terminated")]
    InvalidUniformName

}

impl Fail for GLError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for GLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl GLError {
    pub fn kind(&self) -> GLErrorKind {
        *self.inner.get_context()
    }
}

impl From<GLErrorKind> for GLError {
    fn from(kind: GLErrorKind) -> GLError {
        GLError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<GLErrorKind>> for GLError {
    fn from(inner: Context<GLErrorKind>) -> GLError {
        GLError { inner }
    }
}
