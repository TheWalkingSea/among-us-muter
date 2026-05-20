use std::thread;
use std::time::Duration;

use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_F1};

pub fn spawn_f1_listener<F>(mut on_press: F)
where
    F: FnMut() + Send + 'static,
{
    thread::spawn(move || {
        let mut was_down = false;
        loop {
            let down = unsafe { GetAsyncKeyState(VK_F1 as i32) as u16 & 0x8000 != 0 };
            if down && !was_down {
                on_press();
            }
            was_down = down;
            thread::sleep(Duration::from_millis(30));
        }
    });
}
