use gl::types::*;

pub mod capabilties;

pub fn clear_color(red :f32, green: f32, blue: f32, alpha: f32){
    unsafe{
        gl::ClearColor(red, green, blue, alpha);
    }
}

pub fn viewport(x: i32, y: i32, width: u16, height: u16){
        unsafe {
            gl::Viewport(x, y, i32::from(width), i32::from(height))
        }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DrawMode{
    Points,
    LineStrip,
    LineLoop,
    Lines,
    LineStripAdjacency,
    LinesAdjacency,
    TriangleStrip,
    TriangleFan,
    Triangles,
    TriangleStripAdjacency,
    TrianglesAdjacency,
    Patches
}

impl DrawMode{
    pub fn value(self) -> GLenum{
        use self::DrawMode::*;
        match self{
            Points => gl::POINTS,
            LineStrip => gl::LINE_STRIP,
            LineLoop => gl::LINE_LOOP,
            Lines => gl::LINES,
            LineStripAdjacency => gl::LINE_STRIP_ADJACENCY,
            LinesAdjacency => gl::LINES_ADJACENCY,
            TriangleStrip => gl::TRIANGLE_STRIP,
            TriangleFan => gl::TRIANGLE_FAN,
            Triangles => gl::TRIANGLES,
            TriangleStripAdjacency => gl::TRIANGLE_STRIP_ADJACENCY,
            TrianglesAdjacency => gl::TRIANGLES_ADJACENCY,
            Patches => gl::PATCHES
        }
    }
}

pub enum IndexType{
    UnsignedByte,
    UnsignedShort,
    UnsignedInt
}
impl IndexType{
    pub fn value(self) -> GLenum{
        use self::IndexType::*;
        match self{
            UnsignedByte => gl::UNSIGNED_BYTE,
            UnsignedShort => gl::UNSIGNED_SHORT,
            UnsignedInt => gl::UNSIGNED_INT,
        }
    }
}


pub fn draw_elements(draw_mode: DrawMode, index_count: i32, index_type: IndexType, indices: *const std::os::raw::c_void){
    unsafe{
        gl::DrawElements(draw_mode.value(), index_count as i32, index_type.value(), indices);
    }
}