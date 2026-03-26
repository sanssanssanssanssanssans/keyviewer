use device_query::Keycode;

pub struct KeyLayout {
    pub keys: Vec<Keycode>,
    pub labels: Vec<&'static str>,
}

impl KeyLayout {
    pub fn mania_11k_default() -> Self {
        Self {
            keys: vec![
                Keycode::Q,
                Keycode::W,
                Keycode::E,
                Keycode::R,
                Keycode::V,
                Keycode::Space,
                Keycode::RShift,
                Keycode::BackSlash, 
                Keycode::Delete,
                Keycode::End, 
                Keycode::PageDown,
            ],
            labels: vec![
                "Q", "W", "E", "R", "V", "SPC",
                "RSHIFT", "\\", "DEL", "END", "PGDN",
            ],
        }
    }
}