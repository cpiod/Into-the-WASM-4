use crate::utils::*;
use crate::map::*;
use crate::wasm4::*;
use crate::sprcnst::*;
use crate::*;

pub const SPRITE_WIDTH: u32 = 14;
pub const SPRITE_HEIGHT: u32 = 18;

#[derive(Clone, Copy)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl Direction {
    fn get_x_y(&self) -> (i32,i32) {
        match &self {
            Direction::UP => (0,-1),
            Direction::DOWN => (0,1),
            Direction::LEFT => (-1,0),
            Direction::RIGHT => (1,0),
        }
    }

}

pub type Entity = usize;

pub enum MechTyp {
    Prime, Science, Ranged, Brute
}

impl MechTyp {
    fn get_sprites(&self) -> Sprites {
        match &self {
            MechTyp::Prime => Sprites { x: 0, y: 0},
            MechTyp::Science => Sprites { x: 0, y: 18},
            MechTyp::Brute => Sprites { x: 0, y: 3*18},
            MechTyp::Ranged => Sprites { x: 0, y: 2*18},
        }
    }

    fn does_float(&self) -> bool {
        match &self {
            MechTyp::Science => true,
            _ => false
        }

    }

    fn get_hp(&self) -> Health {
        match &self {
            _ => Health { hp: 1, max_hp: 2 }
        }
    }
}

struct Mech {
    typ: MechTyp
}

// ---------------

pub enum VekTyp {
    Hornet,
    Firefly,
    Scarab,
    Beetle,
    Psion
}

impl VekTyp {
    fn get_sprites(&self) -> Sprites {
        unsafe {print_number((crate::FRAME_NUMBER) as i32,2,150);}
        text("TEST",2,150);
        let x = unsafe { if (FRAME_NUMBER%30) < 15 { 0 } else { 14 } };
        match &self {
            VekTyp::Hornet => Sprites { x: x, y: 4*18},
            VekTyp::Firefly => Sprites { x: x, y: 6*18},
            VekTyp::Scarab => Sprites { x: x, y: 8*18},
            VekTyp::Beetle => Sprites { x: x, y: 7*18},
            VekTyp::Psion => Sprites { x: x, y: 5*18},
        }
    }

    fn does_float(&self) -> bool {
        match &self {
            VekTyp::Psion => true,
            _ => false
        }

    }


    fn get_hp(&self) -> Health {
        match &self {
            _ => Health { hp: 1, max_hp: 1 }
        }
    }
}

struct Vek {
    typ: VekTyp
}

struct Attack {
    target_x: u8,
    target_y: u8,
    typ: AttackTyp
}

pub enum AttackTyp {
    Melee,
    Beam,
    Projectile,
    Artillery
}


// ---------------

struct Sprites {
    x: u8,
    y: u8
}

struct Health {
    hp: i8,
    max_hp: i8
}
struct Position {
    x: u8,
    y: u8
}

enum Weapon {

}

struct Float {}

impl Weapon {
    fn get_minimap_id(&self) -> u8 {
        match self {
            _ => 0
        }
    }
}

struct AI {}

pub const ENTITY_MAX_NUMBER: usize = 10;

type ComponentsArray<T> = [Option<T>; ENTITY_MAX_NUMBER];

#[derive(Default)]
enum GameState {
   #[default]
    TitleScreen,
    WaitingUser,
    VekTurn
}

#[derive(Default)]
pub struct World {
    map: Map,
    state: GameState,
    exists: [bool; ENTITY_MAX_NUMBER],
    health_components: ComponentsArray<Health>,
    position_components: ComponentsArray<Position>,
    mech_components: ComponentsArray<Mech>,
    vek_components: ComponentsArray<Vek>,
    sprites_components: ComponentsArray<Sprites>,
    attacks_components: ComponentsArray<Attack>,
    ai_components: ComponentsArray<AI>,
    float_components: ComponentsArray<Float>,

}

#[inline]
fn _with<T>(entity: Entity, array: &mut ComponentsArray<T>, component: T) {
    if array.get(entity).is_none() {
        loop {}
    }
    array[entity] = Some(component);
}


impl World {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn get_map(&mut self) -> &mut Map {
        &mut self.map
    }

    pub unsafe fn save_world(&self) {
        diskw((self as *const World) as *const u8, core::mem::size_of::<Self>() as u32);
    }

    pub unsafe fn load_world(&mut self) {
        diskr((self as *mut World) as *mut u8, core::mem::size_of::<Self>() as u32);
    }

    pub fn load_map(&mut self, id: usize) {
        self.map = Map::new(id)
    }

    pub fn new_entity(&mut self) -> Entity {
        let index = unwrap_abort(self.exists.iter().position(|&x| x==false));
        self.exists[index] = true;
        index
    }

    pub fn update_pos(&mut self, e: Entity, x: u8, y: u8) {
        if let Some(pos) = &mut self.position_components[e] {
            pos.x = x;
            pos.y = y;
        }
    }

    pub fn spawn_mech(&mut self, t: MechTyp, x: u8, y: u8) -> Entity {
        let e = self.new_entity();
        self.with_spr(e, t.get_sprites());
        self.with_hp(e, t.get_hp() );
        if t.does_float() {
            self.with_float(e);
        }
        self.with_mech(e, Mech { typ: t });
        self.with_pos(e, Position { x: x, y: y });
        e
    }

    pub fn spawn_vek(&mut self, t: VekTyp, x: u8, y: u8) -> Entity {
        let e = self.new_entity();
        self.with_spr(e, t.get_sprites());
        self.with_hp(e, t.get_hp() );
        if t.does_float() {
            self.with_float(e);
        }
        self.with_vek(e, Vek { typ: t});
        self.with_pos(e, Position { x: x, y: y });
        e
    }

    fn with_hp(&mut self, entity: Entity, component: Health) {
        _with(entity, &mut self.health_components, component)
    }

    fn with_pos(&mut self, entity: Entity, component: Position) {
        _with(entity, &mut self.position_components, component)
    }

    fn with_spr(&mut self, entity: Entity, component: Sprites) {
        _with(entity, &mut self.sprites_components, component)
    }

    fn with_mech(&mut self, entity: Entity, component: Mech) {
        _with(entity, &mut self.mech_components, component)
    }

    fn with_vek(&mut self, entity: Entity, component: Vek) {
        _with(entity, &mut self.vek_components, component)
    }

    fn with_float(&mut self, entity: Entity) {
        _with(entity, &mut self.float_components, Float {})
    }


    pub fn atk_ent(&mut self, src: Entity, dst: Entity) {
        if let Some(pos) = &self.position_components[dst] {
            self.atk_pos(src, pos.x, pos.y, AttackTyp::Melee);
        } else {
            panic!()
        }
    }

    pub fn atk_pos(&mut self, src: Entity, dst_x: u8, dst_y: u8, typ: AttackTyp) {
        _with(src, &mut self.attacks_components, Attack { target_x: dst_x, target_y: dst_y, typ: typ })
    }

    fn is_mech(&self, e: Entity) -> bool {
        if !self.exists[e] {
            panic!()
        }
        self.mech_components[e].is_some()
    }

    fn is_vek(&self, e: Entity) -> bool {
        if !self.exists[e] {
            panic!()
        }
        self.vek_components[e].is_some()
    }

    fn del_ent(&mut self, e: Entity) {
        self.exists[e] = false;
        self.health_components[e] = None;
        self.position_components[e] = None;
        self.mech_components[e] = None;
        self.vek_components[e] = None;
        self.sprites_components[e] = None;
        self.attacks_components[e] = None;
    }

    pub fn del_all_veks(&mut self) {
        for index in 0..ENTITY_MAX_NUMBER {
            if self.vek_components[index].is_some() {
                self.del_ent(index)
            }
        }
    }

    pub fn get_mech_at(&self, mouse_sxy: (u8, u8)) -> Option<Entity> {
        for index in 0..ENTITY_MAX_NUMBER {
            if let (Some(pos), Some(_)) = (&self.position_components[index], &self.mech_components[index]) {
                if pos.x == mouse_sxy.0 && pos.y == mouse_sxy.1 {
                    return Some(index)
                }
            }
        }
        return None
    }

    pub unsafe fn render_map(&mut self) -> Option<(i32,i32)> {
        self.map.render()
    }

    pub fn can_traverse(&self, e: Entity, x: u8, y: u8) -> bool {
        if self.float_components[e].is_some() {
            return true;
        } else {
            if self.is_mech(e) {
                for index in 0..ENTITY_MAX_NUMBER {
                    if let (Some(pos), Some(_)) = (&self.position_components[index], &self.vek_components[index]) {
                        if pos.x == x && pos.y == y {
                            return false;
                        }
                    }
                }
                return self.map.can_mech_go(x,y);
            } else if self.is_vek(e) {
                for index in 0..ENTITY_MAX_NUMBER {
                    if let (Some(pos), Some(_)) = (&self.position_components[index], &self.mech_components[index]) {
                        if pos.x == x && pos.y == y {
                            return false;
                        }
                    }
                }
                return self.map.can_vek_go(x,y);
            }
            return false; // no mech, no vek ??
        }
    }

    pub fn do_pathfinding(&self, e: Entity, max_dist: u8) -> [[Option<Direction>;8];8] {
        let mut queue : [Option<(u8,u8,u8)>;36] = [None; 36];
        let mut output : [[Option<Direction>;8];8] = [[None; 8]; 8];
        let dirs = [Direction::UP, Direction::DOWN, Direction::RIGHT, Direction::LEFT];
        if let Some(pos) = &self.position_components[e] { // always true
            queue[0] = Some((pos.x,pos.y,0));
            let mut write_queue_index = 1;
            let mut read_queue_index = 0;
            while read_queue_index < write_queue_index {
                if let Some(current_pos) = queue[read_queue_index] { // alway true
                    read_queue_index += 1;
                    for direction in dirs.iter() {
                        let (dx,dy) = direction.get_x_y();
                        let new_pos = (current_pos.0 as i32 + dx, current_pos.1 as i32 + dy, current_pos.2 + 1);
                        if new_pos.0>=0 && new_pos.0<6 && new_pos.1>=0 && new_pos.1<6 &&
                            output[new_pos.0 as usize][new_pos.1 as usize].is_none() &&
                            new_pos.2 <= max_dist &&
                            self.can_traverse(e, new_pos.0 as u8, new_pos.1 as u8) {
                                queue[write_queue_index] = Some((new_pos.0 as u8, new_pos.1 as u8, new_pos.2));
                                write_queue_index += 1;
                                output[new_pos.0 as usize][new_pos.1 as usize] = Some(direction.clone());
                        }
                    }
                }
            }
        }
        output
    }

    pub fn is_occupied(&self, x: u8, y: u8) -> bool {
        for index in 0..ENTITY_MAX_NUMBER {
            if let Some(pos) = &self.position_components[index] {
                if pos.x == x && pos.y == y {
                    return true
                }
            }
        }
        return false
    }

    pub fn get_path_to<'a>(&self, e: Entity, x: u8, y: u8, path: &'a mut [(u8,u8); 36], pf: [[Option<Direction>;8];8]) -> &'a[(u8,u8)]
    {
        let mut cx = x as i32;
        let mut cy = y as i32;
        let mut index = 36;
        if let Some(pos) = &self.position_components[e] {
            while let Some(dir) = pf[cx as usize][cy as usize] {
                index -= 1;
                path[index] = (cx as u8,cy as u8);
                let xy = dir.get_x_y();
                cx -= xy.0;
                cy -= xy.1;
                if cx == pos.x as i32 && cy == pos.y as i32 {
                    break
                }
            }
        }
        &path[index..]
    }

    // SYSTEMS

    pub unsafe fn render_pathfinding(&self, pf: [[Option<Direction>;8];8] ) {
        *DRAW_COLORS = 0x0040;
        for x in 0..6 {
            for y in 0..6 {
                let (sx,sy) = board_to_screen(x, y);
                if let Some(dir) = pf[x as usize][y as usize] {
                    let flags = match dir {
                        Direction::UP => UI_FLAGS | BLIT_FLIP_X,
                        Direction::DOWN => UI_FLAGS | BLIT_FLIP_Y,
                        Direction::LEFT => UI_FLAGS | BLIT_FLIP_X | BLIT_FLIP_Y,
                        Direction::RIGHT => UI_FLAGS,
                    };
                    blit_sub(&UI, sx+10, sy+3, 8, 10, 15, 0, UI_WIDTH, flags);
                }
            }
        }
    }

    pub unsafe fn sys_render_atks(&self) {
        *DRAW_COLORS = 0x0040;
        for index in 0..ENTITY_MAX_NUMBER {
            if let (Some(pos), Some(atk)) = (&self.position_components[index], &self.attacks_components[index]) {
                let (sx,sy) = board_to_screen(atk.target_x, atk.target_y);
                blit_sub(&UI, sx, sy, 12, 10, 0, 0, UI_WIDTH, UI_FLAGS);
                blit_sub(&UI, sx+12, sy, 12, 10, 0, 0, UI_WIDTH, UI_FLAGS | BLIT_FLIP_X);
                blit_sub(&UI, sx, sy+9, 12, 10, 0, 0, UI_WIDTH, UI_FLAGS | BLIT_FLIP_Y);
                blit_sub(&UI, sx+12, sy+9, 12, 10, 0, 0, UI_WIDTH, UI_FLAGS | BLIT_FLIP_X | BLIT_FLIP_Y);
            }
        }
    }

    unsafe fn render_one_char(&self, index: Entity, pos: &Position, spr: &Sprites) {
        let (sx,mut sy) = board_to_screen(pos.x, pos.y);
        if self.float_components[index].is_some() {
            sy -= 2
        } else if self.map.is_water(pos.x, pos.y) {
            sy += 4
        }
        blit_sub(&CHARS, sx+5, sy-5, SPRITE_WIDTH, SPRITE_HEIGHT, spr.x as u32, spr.y as u32, CHARS_WIDTH, CHARS_FLAGS);
    }

    pub unsafe fn sys_render_chars(&self, anim: &Option<MovingAnimation>) {
        *DRAW_COLORS = 0x4310;
        for index in 0..ENTITY_MAX_NUMBER {
            if let (Some(pos), Some(spr)) = (&self.position_components[index], &self.sprites_components[index]) {
                match anim {
                    Some(anim) if anim.e == index => {
                        let pos = &anim.get_position();
                        self.render_one_char(index, &Position { x: pos.0, y: pos.1 }, spr)
                    },
                    _ => self.render_one_char(index, pos, spr)
                }
            }
        }
    }

    pub unsafe fn sys_render_hp(&self) {
        for index in 0..ENTITY_MAX_NUMBER {
            if let (Some(pos), Some(hp)) = (&self.position_components[index], &self.health_components[index]) {
                let (mut sx,sy) = board_to_screen(pos.x, pos.y);
                sx += 12 - 3*(hp.max_hp as i32);
                for i in 0..hp.max_hp as i32 {
                    if i < hp.hp as i32 {
                        *DRAW_COLORS = 0x0023;
                    } else {
                        *DRAW_COLORS = 0x0021;
                    }
                    rect(sx+5*i, sy-8, 5, 4)
                }
            }
        }
    }
}
