extern crate sdl2;
extern crate gl;
extern crate glm;

pub mod input;
pub mod vbo;
pub mod vao;
pub mod resources;
pub mod shader;
pub mod camera;

use std::path::Path;
use gl::types::*;
use resources::*;
use resources::Error;
use shader::*;


fn main() {

    //---------------------------//
    //-- OpenGL Initialization --//
    //---------------------------//

    let screen_width = 900;
    let screen_height = 700;

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    let window = video_subsystem
        .window("Game", screen_width, screen_height)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with( |s| video_subsystem.gl_get_proc_address(s) as *const _);





    //-------------------------//
    //-- Game Initialization --//
    //-------------------------//
    
    // Input
    let mut input = input::Input::new(sdl.event_pump().unwrap());	

    // Resources
    let res = 
        match Resources::from_relative_exe_path(Path::new("assets")){
            Ok(value) => value,
            Err(_) => panic!("Erro ao abrir diretorio de assets"),
    };
    
    // Shader Program
    let shader_program = 
        match Program::from_res(&res, "shaders/triangle") {
            Ok(value) => value,
            Err(_)    => panic!("Erro ao carregar shaders"),
        };
    
    // Camera 
    let mut camera = camera::Camera::new( glm::vec3(0.0, 0.0, 3.0) );

    // Triangle Vertices
    let vertices: Vec<f32> = vec![
        -0.5, -0.5, -0.5,  0.0, 0.0,
         0.5, -0.5, -0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5,  0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 0.0,

        -0.5, -0.5,  0.5,  0.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 1.0,
        -0.5,  0.5,  0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,

        -0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5, -0.5,  1.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5,  0.5,  1.0, 0.0,

         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5,  0.5,  0.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,

        -0.5, -0.5, -0.5,  0.0, 1.0,
         0.5, -0.5, -0.5,  1.0, 1.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
         0.5, -0.5,  0.5,  1.0, 0.0,
        -0.5, -0.5,  0.5,  0.0, 0.0,
        -0.5, -0.5, -0.5,  0.0, 1.0,

        -0.5,  0.5, -0.5,  0.0, 1.0,
         0.5,  0.5, -0.5,  1.0, 1.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
         0.5,  0.5,  0.5,  1.0, 0.0,
        -0.5,  0.5,  0.5,  0.0, 0.0,
        -0.5,  0.5, -0.5,  0.0, 1.0
    ];

    // Create transform Matrix
    let model: glm::Mat4  = glm::mat4(1.0, 0.0, 0.0, 0.0,
                                      0.0, 1.0, 0.0, 0.0,
                                      0.0, 0.0, 1.0, 0.0,
                                      0.0, 0.0, 0.0, 1.0 
                                     );
    let view: glm::Mat4  = glm::mat4(1.0, 0.0, 0.0, 0.0,
                                     0.0, 1.0, 0.0, 0.0,
                                     0.0, 0.0, 1.0, 0.0,
                                     0.0, 0.0, 0.0, 1.0 
                                     );

    let model = glm::ext::rotate(&model, glm::radians(0.0), glm::vec3(1.0, 0.0, 0.0));
    let view = glm::ext::translate(&view, glm::vec3(0.0, 0.0,-3.0));
    let projection = glm::ext::perspective(glm::radians(45.0), (screen_width/screen_height) as f32, 1.0, 100.0);

    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
        gl::Viewport(0, 0, screen_width as i32, screen_height as i32);
    }


    // Create VBO
    let vbo = vbo::VBO::new(vertices, gl::STATIC_DRAW);
    // Create VAO
    let vao = vao::VAO::new();

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



    //---------------//
    //-- Main Loop --//
    //---------------//
    while !input.get_quit() {

        let delta_time = 0.01;

        input.update();
        camera.update(&mut input, delta_time);

        let view = camera.get_view_matrix();

        shader_program.set_used();
        shader_program.set_float_1f("verde", 1.0);
        shader_program.set_mat4f("model", &model);
        shader_program.set_mat4f("view", &view);
        shader_program.set_mat4f("projection", &projection);

        unsafe{
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Draw
        unsafe {

            vao.bind();
            gl::DrawArrays(
                gl::TRIANGLES,
                0,
                36
            );
        }

        window.gl_swap_window();
    }

}
