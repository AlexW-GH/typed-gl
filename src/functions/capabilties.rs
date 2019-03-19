pub use crate::gl_wrapper::GL;

pub fn enable(capability: Capability){
    unsafe{
        GL::enable(capability.value());
    }
}

pub fn disable(capability: Capability){
    unsafe{
        GL::disable(capability.value());
    }
}

pub fn is_enabled(capability: Capability) -> bool{
    unsafe{
        let result = GL::is_enabled(capability.value());
        result == gl::TRUE
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Capability{
    Blend,
    CullFace,
    DepthTest,
    Dither,
    PolygonOffsetFill,
    SampleAlphaToCoverage,
    SampleCoverage,
    ScissorTest,
    StencilTest,
}

impl Capability{
    pub fn value(self) -> u32{
        use self::Capability::*;
        match self{
            Blend => gl::BLEND,
            CullFace => gl::CULL_FACE,
            DepthTest => gl::DEPTH_TEST,
            Dither => gl::DITHER,
            PolygonOffsetFill => gl::POLYGON_OFFSET_FILL,
            SampleAlphaToCoverage => gl::SAMPLE_ALPHA_TO_COVERAGE,
            SampleCoverage => gl::SAMPLE_COVERAGE,
            ScissorTest => gl::SCISSOR_TEST,
            StencilTest => gl::STENCIL_TEST,
        }

    }
}