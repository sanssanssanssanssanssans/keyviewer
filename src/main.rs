mod core;

use crate::core::{config, tracker, viewer};
use device_query::{DeviceQuery, DeviceState};
use macroquad::prelude::*;

#[cfg(target_os = "windows")]
use winapi::um::winuser::{
    SetWindowPos, GetForegroundWindow,
    HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE,
};

fn window_conf() -> Conf {
    Conf {
        window_title: "키뷰어".to_owned(),
        window_width: 800,
        window_height: 600,
        high_dpi: true,
        ..Default::default()
    }
}

#[cfg(target_os = "windows")]
fn set_always_on_top() {
    unsafe {
        let hwnd = GetForegroundWindow();
        SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE,
        );
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let layout = config::KeyLayout::mania_11k_default();
    let mut tracker = tracker::KeyTracker::new();
    let device_state = DeviceState::new();
    let mut rain_viewer = viewer::RainViewer::new();
    #[cfg(target_os = "windows")]
    set_always_on_top();

    loop {
        let keys = device_state.get_keys();
        tracker.update(keys);
        rain_viewer.update_and_draw(&layout, &tracker);
        next_frame().await;
    }
}