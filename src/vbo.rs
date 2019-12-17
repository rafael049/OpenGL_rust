extern crate gl;

use gl::types::*;
use std::mem::*;

pub struct VBO {
    id: u32,
}


impl VBO {

    pub fn new(vertices: Vec<f32>, usage: GLenum) -> VBO {
        let mut id: u32 = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
            gl::BindBuffer(gl::ARRAY_BUFFER, id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                usage,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }

        VBO { id: id }
    }

    pub fn write_to_buffer( vertices: Vec<f32>, usage: GLenum) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<f32>()) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                usage,
            );
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

}
