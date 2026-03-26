use macroquad::prelude::*;
use super::config::KeyLayout;
use super::tracker::KeyTracker;

#[derive(Clone, Copy)]
pub struct Trail {
    pub y: f32,
    pub length: f32,
}

pub struct RainViewer {
    trails: Vec<Vec<Trail>>, 
    speed: f32,
}

impl RainViewer {
    pub fn new() -> Self {
        Self {
            trails: vec![Vec::new(); 13],
            speed: 800.0, 
        }
    }

    pub fn update_and_draw(&mut self, layout: &KeyLayout, tracker: &KeyTracker) {
        let dt = get_frame_time(); 
        clear_background(Color::new(0.05, 0.05, 0.05, 1.0));
        let screen_w = screen_width();
        let screen_h = screen_height();
        let key_count = layout.keys.len() as f32;
        let key_spacing = 10.0;
        let key_width = (screen_w - key_spacing * (key_count - 1.0)) / key_count;
        let start_x = 0.0;
        let bottom_y = screen_h - 100.0; 
        for (i, key) in layout.keys.iter().enumerate() {
            let is_pressed = tracker.is_pressed(key);
            let x = start_x + (key_width + key_spacing) * i as f32;
            for trail in &mut self.trails[i] {
                trail.y -= self.speed * dt;
            }
            if is_pressed {
                let mut needs_new = true;
                if let Some(last_trail) = self.trails[i].last_mut() {
                    if last_trail.y + last_trail.length >= bottom_y - 2.0 {
                        last_trail.length += self.speed * dt;
                        last_trail.y -= self.speed * dt;
                        needs_new = false;
                    }
                }
                if needs_new {
                    self.trails[i].push(Trail {
                        y: bottom_y - (self.speed * dt),
                        length: self.speed * dt,
                    });
                }
            }
            self.trails[i].retain(|t| t.y + t.length > 0.0);
            let glow_color = if i == 4 || i == 5 || i == 6 { 
                Color::new(1.0, 0.3, 0.5, 0.8) 
            } else { 
                Color::new(0.2, 0.8, 1.0, 0.8) 
            };
            for trail in &self.trails[i] {
                draw_rectangle(x, trail.y, key_width, trail.length, glow_color);
            }
            let button_color = if is_pressed { WHITE } else { GRAY };
            draw_rectangle(x, bottom_y, key_width, 60.0, button_color);
            let label = layout.labels[i];
            let text_color = if is_pressed { BLACK } else { WHITE };
            let font_size = 30.0;
            let text_dim = measure_text(label, None, font_size as u16, 1.0);
            let text_x = x + (key_width - text_dim.width) / 2.0;
            let text_y = bottom_y + (60.0 + text_dim.height) / 2.0;
            draw_text(label, text_x, text_y, font_size, text_color);
        }
    }
}