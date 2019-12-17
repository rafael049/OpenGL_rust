extern crate gl;

use gl::types::*;
use crate::vao::*;
use crate::vbo::*;

pub struct Mesh {
    vao: VAO,
    vbo: VBO,
}


impl Mesh {
    pub fn new(vertices: Vec<f32>) -> Mesh {

        // Create VBO
        let vbo = VBO::new(vertices, gl::STATIC_DRAW);
        // Create VAO
        let vao = VAO::new();

        // COnfigure VBO VAO
        unsafe {
            vao.bind();
            vbo.bind();

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (5 * std::mem::size_of::<f32>()) as GLint,
                std::ptr::null()
            );


            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                (5 * std::mem::size_of::<f32>()) as GLint,
                (3 * std::mem::size_of::<f32>()) as *const GLvoid
            );

            vao.unbind();
            vbo.unbind();
        }

        Mesh { vao: vao, vbo: vbo }
    }

    pub fn draw(&self){
        self.vao.bind();
        unsafe {
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                36
            );
        }
    }
}
