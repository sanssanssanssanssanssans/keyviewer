use device_query::Keycode;
use std::collections::HashSet;

pub struct KeyTracker {
    pressed_keys: HashSet<Keycode>,
}

impl KeyTracker {
    pub fn new() -> Self {
        Self {
            pressed_keys: HashSet::new(),
        }
    }

    pub fn update(&mut self, current_keys: Vec<Keycode>) -> bool {
        let new_pressed: HashSet<Keycode> = current_keys.into_iter().collect();
        
        if self.pressed_keys != new_pressed {
            self.pressed_keys = new_pressed;
            true 
        } else {
            false 
        }
    }

    pub fn is_pressed(&self, key: &Keycode) -> bool {
        self.pressed_keys.contains(key)
    }
}