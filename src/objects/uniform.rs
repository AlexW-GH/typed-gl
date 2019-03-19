use crate::objects::program::GLUniformLocation;
use gl;

pub trait Uniform{
    type Item;
    fn set_value(&self, value: &Self::Item) -> ();
}

impl Uniform for GLUniformLocation<f32>{
    type Item = f32;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform1f(self.program, self.location, *value);
        }
    }
}

impl Uniform for GLUniformLocation<[f32; 2]>{
    type Item = [f32; 2];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform2f(self.program, self.location, value[0], value[1]);
        }
    }
}

impl Uniform for GLUniformLocation<[f32; 3]>{
    type Item = [f32; 3];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform3f(self.program, self.location, value[0], value[1], value[2]);
        }
    }
}

impl Uniform for GLUniformLocation<[f32; 4]>{
    type Item = [f32; 4];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform4f(self.program, self.location, value[0], value[1], value[2], value[3]);
        }
    }
}

impl Uniform for GLUniformLocation<i32>{
    type Item = i32;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform1i(self.program, self.location, *value);
        }
    }
}

impl Uniform for GLUniformLocation<[i32; 2]>{
    type Item = [i32; 2];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform2i(self.program, self.location, value[0], value[1]);
        }
    }
}

impl Uniform for GLUniformLocation<[i32; 3]>{
    type Item = [i32; 3];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform3i(self.program, self.location, value[0], value[1], value[2]);
        }
    }
}

impl Uniform for GLUniformLocation<[i32; 4]>{
    type Item = [i32; 4];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform4i(self.program, self.location, value[0], value[1], value[2], value[3]);
        }
    }
}

impl Uniform for GLUniformLocation<u32>{
    type Item = u32;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform1ui(self.program, self.location, *value);
        }
    }
}

impl Uniform for GLUniformLocation<[u32; 2]>{
    type Item = [u32; 2];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform2ui(self.program, self.location, value[0], value[1]);
        }
    }
}

impl Uniform for GLUniformLocation<[u32; 3]>{
    type Item = [u32; 3];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform3ui(self.program, self.location, value[0], value[1], value[2]);
        }
    }
}

impl Uniform for GLUniformLocation<[u32; 4]>{
    type Item = [u32; 4];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform4ui(self.program, self.location, value[0], value[1], value[2], value[3]);
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [f32]>{
    type Item = &'a [f32];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform1fv(self.program, self.location, value.len() as i32, value.as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[f32; 2]]>{
    type Item = &'a [[f32; 2]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform2fv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[f32; 3]]>{
    type Item = &'a [[f32; 3]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform3fv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[f32; 4]]>{
    type Item = &'a [[f32; 4]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform4fv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [i32]>{
    type Item = &'a [i32];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform1iv(self.program, self.location, value.len() as i32, value.as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[i32; 2]]>{
    type Item = &'a [[i32; 2]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform2iv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[i32; 3]]>{
    type Item = &'a [[i32; 3]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform3iv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[i32; 4]]>{
    type Item = &'a [[i32; 4]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform4iv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [u32]>{
    type Item = &'a [u32];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform1uiv(self.program, self.location, value.len() as i32, value.as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[u32; 2]]>{
    type Item = &'a [[u32; 2]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform2uiv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[u32; 3]]>{
    type Item = &'a [[u32; 3]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform3uiv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[u32; 4]]>{
    type Item = &'a &'a [[u32; 4]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform4uiv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[[f32; 2]; 2]]>{
    type Item = &'a [[[f32; 2]; 2]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix2fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0][0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[[f32; 3]; 3]]>{
    type Item = &'a [[[f32; 3]; 3]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix3fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0][0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[[f32; 4]; 4]]>{
    type Item = &'a [[[f32; 4]; 4]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix4fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0][0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[[f32; 3]; 2]]>{
    type Item = &'a [[[f32; 3]; 2]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix2x3fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0][0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[[f32; 2]; 3]]>{
    type Item = &'a [[[f32; 2]; 3]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix3x2fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0][0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[[f32; 4]; 2]]>{
    type Item = &'a [[[f32; 4]; 2]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix2x4fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0][0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[[f32; 2]; 4]]>{
    type Item = &'a [[[f32; 2]; 4]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix4x2fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0][0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[[f32; 4]; 3]]>{
    type Item = &'a [[[f32; 4]; 3]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix3x4fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0][0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [[[f32; 3]; 4]]>{
    type Item = &'a [[[f32; 3]; 4]];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix4x3fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0][0].as_ptr());
        }
    }
}

use nalgebra::{Vector1, Vector2, Vector3, Vector4, Matrix2, Matrix3, Matrix4, Matrix2x3, Matrix3x2, Matrix2x4, Matrix4x2, Matrix3x4, Matrix4x3, Point1, Point2, Point3, Point4};


impl Uniform for GLUniformLocation<Vector1<f32>>{
    type Item = Vector1<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform1f(self.program, self.location, value.x);
        }
    }
}

impl Uniform for GLUniformLocation<Vector2<f32>>{
    type Item = Vector2<f32>;

    fn set_value(&self, &value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform2f(self.program, self.location, value.x, value.y);
        }
    }
}

impl Uniform for GLUniformLocation<Vector3<f32>>{
    type Item = Vector3<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform3f(self.program, self.location, value.x, value.y, value.z);
        }
    }
}

impl Uniform for GLUniformLocation<Vector4<f32>>{
    type Item = Vector4<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform4f(self.program, self.location, value.x, value.y, value.z, value.w);
        }
    }
}

impl Uniform for GLUniformLocation<Point1<f32>>{
    type Item = Point1<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform1f(self.program, self.location, value.x);
        }
    }
}

impl Uniform for GLUniformLocation<Point2<f32>>{
    type Item = Point2<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform2f(self.program, self.location, value.x, value.y);
        }
    }
}

impl Uniform for GLUniformLocation<Point3<f32>>{
    type Item = Point3<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform3f(self.program, self.location, value.x, value.y, value.z);
        }
    }
}

impl Uniform for GLUniformLocation<Point4<f32>>{
    type Item = Point4<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform4f(self.program, self.location, value.x, value.y, value.z, value.w);
        }
    }
}

impl Uniform for GLUniformLocation<Vector1<i32>>{
    type Item = Vector1<i32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform1i(self.program, self.location, value.x);
        }
    }
}

impl Uniform for GLUniformLocation<Vector2<i32>>{
    type Item = Vector2<i32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform2i(self.program, self.location, value.x, value.y);
        }
    }
}

impl Uniform for GLUniformLocation<Vector3<i32>>{
    type Item = Vector3<i32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform3i(self.program, self.location, value.x, value.y, value.z);
        }
    }
}

impl Uniform for GLUniformLocation<Vector4<i32>>{
    type Item = Vector4<i32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform4i(self.program, self.location, value.x, value.y, value.z, value.w);
        }
    }
}

impl Uniform for GLUniformLocation<Point1<i32>>{
    type Item = Point1<i32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform1i(self.program, self.location, value.x);
        }
    }
}

impl Uniform for GLUniformLocation<Point2<i32>>{
    type Item = Point2<i32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform2i(self.program, self.location, value.x, value.y);
        }
    }
}

impl Uniform for GLUniformLocation<Point3<i32>>{
    type Item = Point3<i32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform3i(self.program, self.location, value.x, value.y, value.z);
        }
    }
}

impl Uniform for GLUniformLocation<Point4<i32>>{
    type Item = Point4<i32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform4i(self.program, self.location, value.x, value.y, value.z, value.w);
        }
    }
}

impl Uniform for GLUniformLocation<Vector1<u32>>{
    type Item = Vector1<u32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform1ui(self.program, self.location, value.x);
        }
    }
}

impl Uniform for GLUniformLocation<Vector2<u32>>{
    type Item = Vector2<u32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform2ui(self.program, self.location, value.x, value.y);
        }
    }
}

impl Uniform for GLUniformLocation<Vector3<u32>>{
    type Item = Vector3<u32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform3ui(self.program, self.location, value.x, value.y, value.z);
        }
    }
}

impl Uniform for GLUniformLocation<Vector4<u32>>{
    type Item = Vector4<u32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform4ui(self.program, self.location, value.x, value.y, value.z, value.w);
        }
    }
}

impl Uniform for GLUniformLocation<Point1<u32>>{
    type Item = Point1<u32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform1ui(self.program, self.location, value.x);
        }
    }
}

impl Uniform for GLUniformLocation<Point2<u32>>{
    type Item = Point2<u32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform2ui(self.program, self.location, value.x, value.y);
        }
    }
}

impl Uniform for GLUniformLocation<Point3<u32>>{
    type Item = Point3<u32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform3ui(self.program, self.location, value.x, value.y, value.z);
        }
    }
}

impl Uniform for GLUniformLocation<Point4<u32>>{
    type Item = Point4<u32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            gl::ProgramUniform4ui(self.program, self.location, value.x, value.y, value.z, value.w);
        }
    }
}

impl Uniform for GLUniformLocation<Matrix2<f32>>{
    type Item = Matrix2<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            gl::ProgramUniformMatrix2fv(self.program, self.location, 1, gl::FALSE, value.as_slice().as_ptr());
        }
    }
}

impl Uniform for GLUniformLocation<Matrix3<f32>>{
    type Item = Matrix3<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix3fv(self.program, self.location, 1, gl::FALSE, value.as_slice().as_ptr());
        }
    }
}

impl Uniform for GLUniformLocation<Matrix4<f32>>{
    type Item = Matrix4<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix4fv(self.program, self.location, 1, gl::FALSE, value.as_slice().as_ptr());
        }
    }
}

impl Uniform for GLUniformLocation<Matrix2x3<f32>>{
    type Item = Matrix2x3<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix2x3fv(self.program, self.location, 1, gl::FALSE, value.as_slice().as_ptr());
        }
    }
}

impl Uniform for GLUniformLocation<Matrix3x2<f32>>{
    type Item = Matrix3x2<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix3x2fv(self.program, self.location, 1, gl::FALSE, value.as_slice().as_ptr());
        }
    }
}

impl Uniform for GLUniformLocation<Matrix2x4<f32>>{
    type Item = Matrix2x4<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix2x4fv(self.program, self.location, 1, gl::FALSE, value.as_slice().as_ptr());
        }
    }
}

impl Uniform for GLUniformLocation<Matrix4x2<f32>>{
    type Item = Matrix4x2<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix4x2fv(self.program, self.location, 1, gl::FALSE, value.as_slice().as_ptr());
        }
    }
}

impl Uniform for GLUniformLocation<Matrix3x4<f32>>{
    type Item = Matrix3x4<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix3x4fv(self.program, self.location, 1, gl::FALSE, value.as_slice().as_ptr());
        }
    }
}

impl Uniform for GLUniformLocation<Matrix4x3<f32>>{
    type Item = Matrix4x3<f32>;

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix4x3fv(self.program, self.location, 1, gl::FALSE, value.as_slice().as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Vector1<f32>]>{
    type Item = &'a [Vector1<f32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform1fv(self.program, self.location, value.len() as i32, value[0].as_slice().as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Vector2<f32>]>{
    type Item = &'a [Vector2<f32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform2fv(self.program, self.location, value.len() as i32, value[0].as_slice().as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Vector3<f32>]>{
    type Item = &'a [Vector3<f32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform3fv(self.program, self.location, value.len() as i32, value[0].as_slice().as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Vector4<f32>]>{
    type Item = &'a [Vector4<f32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform4fv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Vector1<i32>]>{
    type Item = &'a [Vector1<i32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform1iv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Vector2<i32>]>{
    type Item = &'a [Vector2<i32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform2iv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Vector3<i32>]>{
    type Item = &'a [Vector3<i32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform3iv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Vector4<i32>]>{
    type Item = &'a [Vector4<i32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform4iv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Vector1<u32>]>{
    type Item = &'a [Vector1<u32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform1uiv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Vector2<u32>]>{
    type Item = &'a [Vector2<u32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform2uiv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Vector3<u32>]>{
    type Item = &'a [Vector3<u32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform3uiv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Vector4<u32>]>{
    type Item = &'a [Vector4<u32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe {
            let value = *value;
            gl::ProgramUniform4uiv(self.program, self.location, value.len() as i32, value[0].as_ptr())
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Matrix2<f32>]>{
    type Item = &'a [Matrix2<f32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix2fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Matrix3<f32>]>{
    type Item = &'a [Matrix3<f32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix3fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Matrix4<f32>]>{
    type Item = &'a [Matrix4<f32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix4fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Matrix2x3<f32>]>{
    type Item = &'a [Matrix2x3<f32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix2x3fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Matrix3x2<f32>]>{
    type Item = &'a [Matrix3x2<f32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix3x2fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Matrix2x4<f32>]>{
    type Item = &'a [Matrix2x4<f32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix2x4fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Matrix4x2<f32>]>{
    type Item = &'a [Matrix4x2<f32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix4x2fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Matrix3x4<f32>]>{
    type Item = &'a [Matrix3x4<f32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix3x4fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0].as_ptr());
        }
    }
}

impl <'a> Uniform for GLUniformLocation<&'a [Matrix4x3<f32>]>{
    type Item = &'a [Matrix4x3<f32>];

    fn set_value(&self, value: &Self::Item) -> () {
        unsafe{
            let value = *value;
            gl::ProgramUniformMatrix4x3fv(self.program, self.location, value.len() as i32, gl::FALSE, value[0].as_ptr());
        }
    }
}





