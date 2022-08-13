use crate::wasm4::*;

const MAP_RENDER_DX: i32 = 2;
const MAP_RENDER_DY: i32 = 50;

#[inline]
pub fn unwrap_abort<T>(o: Option<T>) -> T {
    match o {
        Some(t) => t,
        None => loop {}
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

pub fn board_to_screen(x: u8, y: u8) -> (i32, i32) {
    let x = x as i32;
    let y = y as i32;
    (12*(x+y)+MAP_RENDER_DX, 9*(y-x)+MAP_RENDER_DY)
}

pub fn screen_to_board(mx: i32, my: i32) -> Option<(u8, u8)> {
    let mut closest = None;
    let mut min_dist = 13;
    for x in 0usize..6 {
        for y in 0usize..6 {
            let (sx, sy) = ((12*(x+y) as i32 + MAP_RENDER_DX), (9*(y-x) as i32 + MAP_RENDER_DY));
            let mx = mx - sx - 12;
            let my = my - sy - 9;
            if mx.abs() + my.abs() < min_dist {
                min_dist = mx.abs() + my.abs();
                closest = Some((x as u8,y as u8));
            }
        }
    }
    closest
}

pub unsafe fn print_number(n: i32, x: i32, y: i32) { // use the current DRAW_COLORS
    let neg = n < 0;
    const ARRAY_LENGTH: usize = 11; // 11 chars are enough for i32 (10 digits max + sign)
    let mut n = n.abs();
    let mut buffer = [0u8; ARRAY_LENGTH];
    let mut i = ARRAY_LENGTH;
    loop { // display at least one number (i.e., display 0 when n=0)
        i -= 1;
        buffer[i] = '0' as u8 + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break
        }
    }
    if neg {
        i -= 1;
        buffer[i] = '-' as u8;
    }
    let string = core::str::from_utf8_unchecked(&buffer[i..ARRAY_LENGTH]);
    text(string, x, y);
}
