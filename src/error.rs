use failure::Fail;
use failure::Context;
use std::fmt;
use failure::Backtrace;

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
        GLError { inner: Context::new(kind) }
    }
}

impl From<Context<GLErrorKind>> for GLError {
    fn from(inner: Context<GLErrorKind>) -> GLError {
        GLError { inner }
    }
}