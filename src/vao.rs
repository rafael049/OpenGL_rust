extern crate gl;

use gl::types::*;
use std::mem::*;

pub struct VAO {
    id: u32,
//    vertex: bool,
//    color:  bool,
//    uv:     bool,
}

impl VAO {

    pub fn new() -> VAO {
        let mut id: u32 = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }

        VAO { id: id }
    }

    pub fn bind(&self){
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

}
