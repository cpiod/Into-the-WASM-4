#![no_std]
// #[cfg(feature = "buddy-alloc")]

mod wasm4;
mod map;
mod utils;
mod ecs;
mod sprcnst;
mod btn;
use utils::*;
use wasm4::*;
use ecs::*;
use sprcnst::*;
use btn::*;

#[no_mangle]
unsafe fn start() {
// palette :
// blanc clair pour le terrain
// noir foncé pour l’arrière plan
// vert assez foncé pour les personnages (avec le blanc)
// rouge assez clair pour tout ce qui requiert l’attention du joueur

    *PALETTE = [
        0x303841,
        0xDBE0DA,
        0x317931,
        0xFF3131,
    ];

    // *PALETTE = [
    //     0x252A34,
    //     0xEAEAEA,
    //     0x08D9D6,
    //     0xFF2E63,
    // ];
    let mut world = World::new();
    world.load_map(0);
    world.spawn_mech(MechTyp::Prime,5,5);
    world.spawn_mech(MechTyp::Science,3,5);
    world.spawn_mech(MechTyp::Brute,0,4);
    world.spawn_mech(MechTyp::Ranged,0,1);
    let e = world.spawn_vek(VekTyp::Hornet,2,2);
    let e2 = world.spawn_vek(VekTyp::Psion,4,0);
    world.spawn_vek(VekTyp::Scarab,4,2);
    world.spawn_vek(VekTyp::Beetle,2,0);
    world.spawn_vek(VekTyp::Firefly,3,4);
    world.save_world();
    GLOBAL_WORLD = Some(world);
}

unsafe fn hilite_mouse(mouse_sxy: Option<(i32, i32, u8, u8)>) {
    if let Some((sx,sy,x,y)) = mouse_sxy {
        if REACHABLE[x as usize][y as usize] {
            *DRAW_COLORS = 0x0010;
        } else {
            *DRAW_COLORS = 0x0030;
        }
        blit_sub(&UI, sx, sy, 12, 10, 0, 0, UI_WIDTH, UI_FLAGS);
        blit_sub(&UI, sx+12, sy, 12, 10, 0, 0, UI_WIDTH, UI_FLAGS | BLIT_FLIP_X);
        blit_sub(&UI, sx, sy+9, 12, 10, 0, 0, UI_WIDTH, UI_FLAGS | BLIT_FLIP_Y);
        blit_sub(&UI, sx+12, sy+9, 12, 10, 0, 0, UI_WIDTH, UI_FLAGS | BLIT_FLIP_X | BLIT_FLIP_Y);
    }
}

unsafe fn draw_ui() {
    *DRAW_COLORS = 0x0012;
    text("Reset",1,110);
    text("Order",160-8*5-1,110);
    text("End",160-3*8-1,1);
}

static mut GLOBAL_WORLD: Option<World> = None;
static mut PREVIOUS_MOUSE: u8 = 0;
static mut FRAME_NUMBER: u32 = 0;
static mut SELECTED_ENTITY: Option<Entity> = None;
static mut PATHFINDING_MAP: [[Option<Direction>;6];6] = [[None; 6]; 6];
static mut REACHABLE: [[bool;6];6] = [[false; 6]; 6];
static mut MOVING_ANIMATION: Option<MovingAnimation> = None;
static mut PATH: [(u8,u8);36] = [(0,0); 36];
static mut TARGETS: Option<[[bool;6];6]> = None;
static mut UNDO_LIST: [Option<(Entity,u8,u8)>;3] = [None;3];


pub struct MovingAnimation {
    e: Entity,
    time: i32,
    path: &'static [(u8,u8)]
}

impl MovingAnimation {
    fn update(&mut self) {
        self.time += 1;
    }

    fn get_position(&self) -> (u8,u8) {
        self.path[(self.time/10) as usize]
    }

    fn get_last_position(&self) -> (u8,u8) {
        self.path[self.path.len()-1]
    }

    fn should_destroy(&self) -> bool {
        (self.time/10) as usize >= self.path.len()
    }

}

unsafe fn update_clicks() -> (bool, bool){
    let previous = PREVIOUS_MOUSE;
    let buttons = *MOUSE_BUTTONS;
    let pressed_this_frame = buttons & (buttons ^ previous);
    PREVIOUS_MOUSE = buttons;
    ((pressed_this_frame & MOUSE_LEFT) != 0, (pressed_this_frame & MOUSE_RIGHT) != 0)
}

unsafe fn undo() {
    if let Some(world) = &mut GLOBAL_WORLD {
        for i in 0..3 {
            if let Some((e,x,y)) = UNDO_LIST[2-i] {
                UNDO_LIST[2-i] = None;
                world.restore_pos(e,x,y);
                SELECTED_ENTITY = None;
                PATHFINDING_MAP = [[None; 6]; 6];
                REACHABLE = [[false; 6]; 6];
                TARGETS = None;
                break
            }
        }
        // world.load_map(0);
        // world.get_map().start_disappearing();
    }
}

unsafe fn update_possible_attacks(world: &World, e: Entity) {
    let attacks = world.get_possible_attacks_no_move(e);
    let mut t = [[false;6];6];
    for atk in attacks.iter() {
        if let Some(atk) = atk {
            if let Some((x,y)) = world.get_attack_target_no_move(e, atk) {
                t[x as usize][y as usize] = true;
            }
        }
    }
    TARGETS = Some(t);
}

#[no_mangle]
unsafe fn update() {
    FRAME_NUMBER += 1;
    if let Some(world) = &mut GLOBAL_WORLD {
        let (left_click, right_click) = update_clicks();
        // world.atk_ent(e,m);
        // world.del_all_veks();
        let mouse_sxy = world.render_map(REACHABLE,TARGETS);
        if left_click {
            let mx=*MOUSE_X as i32;
            let my=*MOUSE_Y as i32;

            let mouse_xy = screen_to_board(mx, my);
            if let Some(mouse_xy) = mouse_xy {
                if let Some(current_e) = world.get_mech_at(mouse_xy) {
                    if world.can_move(current_e) {
                        SELECTED_ENTITY = Some(current_e);
                        (PATHFINDING_MAP, REACHABLE) = world.do_pathfinding(current_e, 5);
                    } else {
                        SELECTED_ENTITY = None;
                        REACHABLE = [[false; 6]; 6];
                        PATHFINDING_MAP = [[None; 6]; 6];
                    }
                    update_possible_attacks(&world, current_e);
                } else if SELECTED_ENTITY.is_some() {
                    if !REACHABLE[mouse_xy.0 as usize][mouse_xy.1 as usize] {
                        SELECTED_ENTITY = None;
                        PATHFINDING_MAP = [[None; 6]; 6];
                        REACHABLE = [[false; 6]; 6];
                        TARGETS = None;
                    }
                    else {
                        let e = unwrap_abort(SELECTED_ENTITY);
                        let p = world.get_path_to(e, mouse_xy.0, mouse_xy.1, &mut PATH, PATHFINDING_MAP);
                        MOVING_ANIMATION = Some( MovingAnimation { e: e, time: 0, path: p });
                        PATHFINDING_MAP = [[None; 6]; 6];
                        REACHABLE = [[false; 6]; 6];
                        TARGETS = None;
                    }
                }
            }
        } else if right_click {
            if SELECTED_ENTITY.is_some() {
                SELECTED_ENTITY = None;
                PATHFINDING_MAP = [[None; 6]; 6];
                REACHABLE = [[false; 6]; 6];
                TARGETS = None;
            }

        }
        let btn = Button::new(1,1,"Undo",0x0012,0x0021,undo);
        btn.draw();
        draw_ui();
        // world.sys_render_atks();
        hilite_mouse(mouse_sxy);
        world.sys_render_chars(&mut MOVING_ANIMATION);
        // world.render_pathfinding(PATHFINDING_MAP);
        // world.sys_render_hp();
        *DRAW_COLORS = 0x0012;
        print_number(core::mem::size_of::<World>() as i32,2,140);
        if left_click {
            btn.check_callback();
        } else if right_click {
            let mut m = world.get_map();
            m.destroy_tile(4,4);
            m.start_appearing();
        }

        let mut animation_ended = false;
        if let Some(anim) = &mut MOVING_ANIMATION {
            anim.update();
            animation_ended = anim.should_destroy();
            if animation_ended {
                let (x,y) = anim.get_last_position();
                let (prev_x, prev_y) = world.update_pos(anim.e, x, y);
                for i in 0..3 {
                    if UNDO_LIST[i].is_none() {
                        UNDO_LIST[i] = Some((anim.e, prev_x, prev_y));
                        break
                    }
                }

                PATHFINDING_MAP = [[None; 6]; 6];
                REACHABLE = [[false; 6]; 6];
                update_possible_attacks(&world, anim.e);
            }
        }
        if animation_ended {
            MOVING_ANIMATION = None;
        }
    }
}
