extern crate sdl2;
extern crate glm;


use crate::input::*;

pub enum CameraDirection {
    Forward,
    Backward,
    Left,
    Right,
}

pub struct Camera {
    pos:      glm::Vector3<f32>,
    front:    glm::Vector3<f32>,
    up:       glm::Vector3<f32>,
    right:    glm::Vector3<f32>,
    worldup:  glm::Vector3<f32>,

    yaw:         f32,  
    pitch:       f32,
    mov_speed:   f32,
    sensivity:   f32,

}

impl Camera {
    pub fn new(pos: glm::Vector3<f32>) -> Camera {
        let yaw       = -90.0;
        let pitch     = 0.0;
        let mov_speed = 2.5;
        let sensivity = 0.2;
        
        let worldup = glm::vec3(0.0, 1.0, 0.0);

        let front = glm::vec3(
            f32::cos(glm::radians(yaw)) * f32::cos(glm::radians(pitch)),
            f32::sin(glm::radians(pitch)),
            f32::sin(glm::radians(yaw)) * f32::cos(glm::radians(pitch)),
        );
        let front = glm::normalize(front);
        
        let right = glm::normalize(glm::cross(front, worldup));

        let up = glm::normalize(glm::cross(right, front));
        Camera{
            pos:       pos,
            front:     front,
            up:        up,
            right:     right,
            worldup:   worldup,
            yaw:       yaw,
            pitch:     pitch,
            mov_speed: mov_speed,
            sensivity: sensivity,
        }
    }

    pub fn get_view_matrix(&self) -> glm::Mat4 {
        glm::ext::look_at(self.pos, self.pos + self.front, self.up)
    }

    pub fn process_input(&mut self, dir: CameraDirection, delta: f32) {
        let velocity = self.mov_speed * delta;
        match dir {
            CameraDirection::Forward  => self.pos = self.pos + self.front*velocity,
            CameraDirection::Backward => self.pos = self.pos - self.front*velocity,
            CameraDirection::Left     => self.pos = self.pos - self.right*velocity,
            CameraDirection::Right    => self.pos = self.pos + self.right*velocity,
        }
    }

    pub fn process_mouse(&mut self, xoffset: f32, yoffset: f32){
        self.yaw += xoffset * self.sensivity;
        self.pitch += yoffset * -self.sensivity;
    }


    pub fn update(&mut self, input: &mut Input, delta_time: f32 ) {

        let front = glm::vec3(
            f32::cos(glm::radians(self.yaw)) * f32::cos(glm::radians(self.pitch)),
            f32::sin(glm::radians(self.pitch)),
            f32::sin(glm::radians(self.yaw)) * f32::cos(glm::radians(self.pitch)),
        );
        self.front = glm::normalize(front);
        
        self.right = glm::normalize(glm::cross(self.front, self.worldup));

        self.up = glm::normalize(glm::cross(self.right, self.front));

        if input.get_key(sdl2::keyboard::Keycode::W){
            self.process_input(CameraDirection::Forward, delta_time);
        }
        if input.get_key(sdl2::keyboard::Keycode::S){
            self.process_input(CameraDirection::Backward, delta_time);
        }
        if input.get_key(sdl2::keyboard::Keycode::A){
            self.process_input(CameraDirection::Left, delta_time);
        }
        if input.get_key(sdl2::keyboard::Keycode::D){
            self.process_input(CameraDirection::Right, delta_time);
        }

        if input.get_mouse_btn(sdl2::mouse::MouseButton::Right) {
            self.process_mouse(input.get_mouse_motion_x(), input.get_mouse_motion_y());
        }
    }
}
