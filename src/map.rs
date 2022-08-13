use crate::wasm4::*;
use crate::*;
use crate::sprcnst::*;
use core::cmp::*;

#[derive(Clone, Copy)]
enum TileType {
    Dirt,
    Water,
    Mountain,
    City,
    NoTile
}

impl TileType {
    fn can_mech_go(&self) -> bool {
        match &self {
            TileType::Dirt | TileType::Water => true,
            _ => false
        }
    }
    fn can_vek_go(&self) -> bool {
        match &self {
            TileType::Dirt => true,
            _ => false
        }
    }

}

const MAP: [[[TileType; 6]; 6]; 1] =
  [[[TileType::Mountain,TileType::Mountain, TileType::Dirt,     TileType::Dirt,     TileType::Dirt,     TileType::Mountain],
    [TileType::Dirt,    TileType::Mountain, TileType::Dirt,     TileType::Dirt,     TileType::Dirt,     TileType::Water],
    [TileType::Dirt,    TileType::Dirt,     TileType::Dirt,     TileType::Dirt,     TileType::Dirt,     TileType::Water],
    [TileType::Mountain,TileType::Dirt,     TileType::Dirt,     TileType::Dirt,     TileType::Dirt,     TileType::Water],
    [TileType::Dirt,    TileType::Dirt,     TileType::Dirt,     TileType::Dirt,     TileType::Water,    TileType::Water],
    [TileType::Dirt,    TileType::Dirt,     TileType::Water,    TileType::Water,    TileType::Water,    TileType::Water]]];

const MINIMAP: [[[TileType; 6]; 6]; 0] = [];

enum MapAnimation {
    NoAnimation,
    Hidden,
    Appearing(u8),
    Disappearing(u8)
}

pub struct Map {
    map: [[TileType; 6]; 6],
    is_minimap: bool,
    animation: MapAnimation
}

impl Default for Map {
    fn default() -> Self {
        Map::new(0)
    }
}

impl Map {
    pub fn can_mech_go(&self, x: u8, y: u8) -> bool {
        self.map[y as usize][x as usize].can_mech_go()
    }

    pub fn can_vek_go(&self, x: u8, y: u8) -> bool {
        self.map[y as usize][x as usize].can_vek_go()
    }

    pub fn new(id: usize) -> Self {
        Map { map : MAP[id], is_minimap: false, animation: MapAnimation::NoAnimation }
    }

    pub unsafe fn render_minimap(id: usize) {
        Map { map : MINIMAP[id], is_minimap: true, animation: MapAnimation::NoAnimation }.render();
    }

    pub fn start_appearing(&mut self) {
        self.animation = MapAnimation::Appearing(100)
    }

    pub fn start_disappearing(&mut self) {
        self.animation = MapAnimation::Disappearing(100)
    }

    pub fn destroy_tile(&mut self, x: usize, y: usize) {
        self.map[x][y]=TileType::NoTile
    }

    pub fn update_map_anim(&mut self) {
        self.animation = match self.animation {
            MapAnimation::Appearing(v) if v >0 => MapAnimation::Appearing(v-1),
            MapAnimation::Disappearing(v) if v > 0 => MapAnimation::Disappearing(v-1),
            MapAnimation::Disappearing(_) => MapAnimation::Hidden,
            _ => MapAnimation::NoAnimation
        }
    }

    pub unsafe fn render(&mut self) -> Option<(i32,i32)> {
        if matches!(self.animation, MapAnimation::Hidden) {
            return None
        }
        *DRAW_COLORS = 0x0021;
        let mx=*MOUSE_X as i32;
        let my=*MOUSE_Y as i32;
        let mut closest = None;
        let mut min_dist=13;
        for sum in -5i32..=5 {
            for x in 0i32..6 {
                let y=sum+x;
                let dy = match self.animation {
                    MapAnimation::Appearing(v) => min(-2*(v as i32)-30*(y-5),0),
                    MapAnimation::Disappearing(v) => max(4*(100-v as i32)+30*(y-5),0),
                    _ => 0
                };
                if y>=0 && ((self.is_minimap && y<3 && x<3) || (!self.is_minimap && y<6)) {
                    let (sx, mut sy) = board_to_screen(x as u8,y as u8);
                    sy += dy;
                    let n = self.map[y as usize][x as usize];
                    match n {
                        TileType::Water => {
                            sy+=4;
                            blit_sub(&ATLAS, sx, sy, 12, 22, 12, 0, ATLAS_WIDTH, ATLAS_FLAGS);
                            blit_sub(&ATLAS, sx+12, sy, 12, 22, 12, 0, ATLAS_WIDTH, ATLAS_FLAGS | BLIT_FLIP_X);
                        },
                        _ => {
                            blit_sub(&ATLAS, sx, sy, 12, 26, 0, 0, ATLAS_WIDTH, ATLAS_FLAGS);
                            blit_sub(&ATLAS, sx+12, sy, 12, 26, 0, 0, ATLAS_WIDTH, ATLAS_FLAGS | BLIT_FLIP_X);
                        }
                    }
                    if matches!(n,TileType::Mountain) { // mountain
                        blit_sub(&ATLAS, sx, sy-4, 24, 19, 24, 3, ATLAS_WIDTH, ATLAS_FLAGS);
                    }
                    let dist = (mx - sx - 12).abs() + (my - sy - 9).abs();
                    if dist < min_dist {
                        min_dist = dist;
                        closest = Some((sx,sy));
                    }
                }
            }
        }
        self.update_map_anim();
        closest
    }
}
