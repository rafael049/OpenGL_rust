extern crate sdl2;

use std::collections::HashMap;

pub struct Input {
    event_pump:        sdl2::EventPump,
    keys_map:          HashMap<sdl2::keyboard::Keycode, bool>,
    mouse_buttons_map: HashMap<sdl2::mouse::MouseButton, bool>,
    mouse_motion:      (f32, f32),
    quit:              bool,
}


impl Input {

    pub fn new(event_pump: sdl2::EventPump) -> Input {
        Input { event_pump:        event_pump,
                keys_map:          HashMap::new(),
                mouse_buttons_map: HashMap::new(),
                mouse_motion:      (0.0, 0.0),
                quit:              false,
              }
    }

    pub fn update(&mut self) {
        self.mouse_motion = (0.0, 0.0);

        for event in self.event_pump.poll_iter() {
            match event {
                sdl2::event::Event::KeyDown {keycode: Some(key), ..} => {self.keys_map.insert(key, true);},
                sdl2::event::Event::KeyUp {keycode: Some(key), ..}   => {self.keys_map.remove(&key);},

                sdl2::event::Event::MouseButtonDown { mouse_btn, ..} => { self.mouse_buttons_map.insert(mouse_btn, true); },
                sdl2::event::Event::MouseButtonUp { mouse_btn, ..}   => { self.mouse_buttons_map.remove(&mouse_btn); },

                sdl2::event::Event::MouseMotion { xrel, yrel, ..}    => { self.mouse_motion = (xrel as f32, yrel as f32); },

                sdl2::event::Event::Quit {..} 			     		 => self.quit = true,

                _                                                    => {},
            }
        }
    }
    pub fn get_key(&mut self, key: sdl2::keyboard::Keycode) -> bool {
        match self.keys_map.get(&key) {
            Some(_) => true,
            _       => false,
        }
    }

    pub fn get_mouse_btn(&mut self, btn: sdl2::mouse::MouseButton) -> bool {
        match self.mouse_buttons_map.get(&btn) {
            Some(_) => true,
            _       => false,
        }
    }

    pub fn get_quit(&mut self) -> bool {
        self.quit
    }

    pub fn get_mouse_motion_x(&self) -> f32 {
        self.mouse_motion.0
    }

    pub fn get_mouse_motion_y(&self) -> f32 {
        self.mouse_motion.1
    }

}
