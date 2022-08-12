use crate::utils::*;
use crate::map::*;

pub type Entity = usize;

pub enum MechTyp {
    Prime, Science, Ranged, Brute
}

struct Mech {
    typ: MechTyp
}

pub enum VekTyp {
    Hornet,
    Firefly,
    Scarab,
    Beetle,
    Psion
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

    pub fn new_entity(&mut self) -> Entity {
        let index = unwrap_abort(self.exists.iter().position(|&x| x==false));
        self.exists[index] = true;
        index
    }

    fn with_hp(&mut self, entity: Entity, component: Health) {
        _with(entity, &mut self.health_components, component)
    }

    fn with_pos(&mut self, entity: Entity, component: Position) {
        _with(entity, &mut self.position_components, component)
    }

    pub unsafe fn render_map(&mut self) {
        self.map.render()
    }

}
