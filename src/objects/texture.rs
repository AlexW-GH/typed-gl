use gl::types::*;

pub struct GLTexture{
    name: GLuint,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextureTarget{
    Texture1D,
    Texture2D,
    Texture3D,
    Texture1DArray,
    Texture2DArray,
    TextureRectangle,
    TextureCubeMap,
    TextureCubeMapArray,
    TextureBuffer,
    Texture2DMultisample,
    Texture2DMultisampleArray
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MipmapTextureTarget{
    Texture1D,
    Texture2D,
    Texture3D,
    Texture1DArray,
    Texture2DArray,
    TextureCubeMap,
    TextureCubeMapArray,
}


pub unsafe fn generate_mipmap(target: MipmapTextureTarget){
    unsafe {
        gl::GenerateMipmap(target.value())
    }
}

impl TextureTarget{
    fn value(&self) -> GLenum{
        use self::TextureTarget::*;
        match self{
            Texture1D => gl::TEXTURE_1D,
            Texture2D => gl::TEXTURE_2D,
            Texture3D => gl::TEXTURE_3D,
            Texture1DArray => gl::TEXTURE_1D_ARRAY,
            Texture2DArray => gl::TEXTURE_2D_ARRAY,
            TextureRectangle => gl::TEXTURE_RECTANGLE,
            TextureCubeMap => gl::TEXTURE_CUBE_MAP,
            TextureCubeMapArray => gl::TEXTURE_CUBE_MAP_ARRAY,
            TextureBuffer => gl::TEXTURE_BUFFER,
            Texture2DMultisample => gl::TEXTURE_2D_MULTISAMPLE,
            Texture2DMultisampleArray => gl::TEXTURE_2D_MULTISAMPLE_ARRAY
        }
    }
}

impl MipmapTextureTarget{
    fn value(&self) -> GLenum{
        use self::MipmapTextureTarget::*;
        match self{
            Texture1D => gl::TEXTURE_1D,
            Texture2D => gl::TEXTURE_2D,
            Texture3D => gl::TEXTURE_3D,
            Texture1DArray => gl::TEXTURE_1D_ARRAY,
            Texture2DArray => gl::TEXTURE_2D_ARRAY,
            TextureCubeMap => gl::TEXTURE_CUBE_MAP,
            TextureCubeMapArray => gl::TEXTURE_CUBE_MAP_ARRAY,
        }
    }
}

impl GLTexture{
    pub fn new() -> GLTexture{
        let mut name = 0;
        unsafe {
            gl::GenTextures(1, &mut name);
        }
        GLTexture{name}
    }

    pub unsafe fn bind(&self, target: TextureTarget, bind: bool){
        if bind {
            unsafe {
                gl::BindTexture(target.value(), self.name);
            }
        } else {
            unsafe {
                gl::BindTexture(target.value(), 0)
            }
        }
    }

    pub fn name(&self) -> u32 {
        self.name
    }
}

impl Drop for GLTexture {
    fn drop(&mut self) {
        unsafe{
            gl::DeleteTextures(1, &mut self.name);
        }
    }

}

pub struct TexParam;

impl TexParam{
    pub unsafe fn depth_stencil_mode(target: TextureTarget){

    }

    pub unsafe fn base_level(target: TextureTarget){

    }

    pub unsafe fn max_level(target: TextureTarget){

    }

    pub unsafe fn swizzle_r(target: TextureTarget){

    }

    pub unsafe fn swizzle_g(target: TextureTarget){

    }

    pub unsafe fn swizzle_b(target: TextureTarget){

    }

    pub unsafe fn swizzle_a(target: TextureTarget){

    }

    pub unsafe fn swizzle_rgba(target: TextureTarget){

    }

    pub unsafe fn lod_bias(target: TextureTarget){

    }

    pub unsafe fn min_filter(target: TextureTarget){

    }

    pub unsafe fn mag_filter(target: TextureTarget){

    }

    pub unsafe fn min_lod(target: TextureTarget){

    }

    pub unsafe fn max_lod(target: TextureTarget){

    }

    pub unsafe fn wrap_s(target: TextureTarget){

    }

    pub unsafe fn wrap_t(target: TextureTarget){

    }

    pub unsafe fn wrap_r(target: TextureTarget){

    }

    pub unsafe fn border_color(target: TextureTarget){

    }

    pub unsafe fn compare_mode(target: TextureTarget){

    }

    pub unsafe fn compare_func(target: TextureTarget){

    }
}