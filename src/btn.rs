use crate::wasm4::*;

pub struct Button<'a> {
    x1: i16,
    y1: i16,
    x2: i16,
    y2: i16,
    text: &'a str,
    inactive_color: u16,
    active_color: u16,
    callback: unsafe fn()
}

impl<'a> Button<'a> {

    pub fn new(x: i16, y: i16, text: &'a str, inactive_color: u16, active_color: u16, callback: unsafe fn()) -> Button<'a> {
        Self {
            x1: x,
            x2: x+(8*text.len() as i16),
            y1: y,
            y2: y+8,
            text: text,
            inactive_color: inactive_color,
            active_color: active_color,
            callback: callback
        }
    }

    unsafe fn is_mouse_over(&self) -> bool {
        let mx=*MOUSE_X;
        let my=*MOUSE_Y;
        mx >= self.x1 && mx < self.x2 && my >= self.y1 && my < self.y2
    }

    pub unsafe fn check_callback(&self) {
        if self.is_mouse_over() {
            (self.callback)()
        }
    }

    pub unsafe fn draw(&self) {
        if self.is_mouse_over() {
            *DRAW_COLORS = self.active_color;
        } else {
            *DRAW_COLORS = self.inactive_color;
        }
        text(self.text,self.x1 as i32,self.y1 as i32);
    }
}
