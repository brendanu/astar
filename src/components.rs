use crate::entities::spawn_bullet;
use crate::geom::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct ShipsSpriteHandles {
    pub handles: Vec<HandleUntyped>,
    pub atlas_loaded: bool,
}
#[derive(Debug)]
pub struct ShipBuilder;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum POWERUPTYPE {
    Health,
    WeaponIncreaseBullets,
    WeaponMultiFire,
}

pub struct PowerUp {
    pub powerup_type: POWERUPTYPE,
    pub time_remaining: f32,
    pub col_shape: Rectangle,
    pub angle: f32,
}

pub enum WEAPONTYPE {
    Single,
    _Multi,
}

pub struct Weapons {
    _weapon_type: WEAPONTYPE,
    pub weapon_cool_down: f32,
    pub max_bullets: u32,
    pub current_bullets: u32,
    pub bullet_type: BULLETTYPE,
}

impl Weapons {
    pub fn new() -> Weapons {
        Weapons {
            weapon_cool_down: 0.0,
            max_bullets: 150,
            current_bullets: 0,
            _weapon_type: WEAPONTYPE::Single,
            bullet_type: BULLETTYPE::Regular,
        }
    }

    pub fn fire(
        &mut self,
        x: f32,
        y: f32,
        angle: f32,
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut asset_server: Res<AssetServer>,
        audio: Res<Audio>,
    ) {
        let sfx: bevy::prelude::Handle<bevy::prelude::AudioSource> =
            asset_server.load("sounds/sfx/sfx_wpn_laser8.ogg");
        audio.play(sfx);

        let (bullets, temp) = spawn_bullet(
            &mut commands,
            &mut materials,
            &mut asset_server,
            x,
            y,
            angle,
            crate::components::OWNERTYPE::Player,
            self.bullet_type,
        );
        self.fired(bullets, temp);
    }

    pub fn change(&mut self, bullet_type: BULLETTYPE) {
        self.bullet_type = bullet_type;
    }

    pub fn can_fire(&self) -> bool {
        self.current_bullets < self.max_bullets && self.weapon_cool_down <= 1.0
    }

    pub fn fired(&mut self, bullets: u32, temp: f32) {
        self.current_bullets += bullets;
        self.weapon_cool_down += temp;
    }

    pub fn cooldown(&mut self) {
        self.weapon_cool_down -= 0.01;
    }
}

pub struct StructuralSystem {
    health: f32,
    max_health: f32,
    shields: f32,
    warning: bool,
}

impl StructuralSystem {
    pub fn new() -> StructuralSystem {
        StructuralSystem {
            health: 20.0,
            shields: 20.0,
            warning: false,
            max_health: 20.0,
        }
    }

    pub fn health_warning(&mut self) -> bool {
        if self.health <= 10.0 && !self.warning {
            self.warning = true;
            return true;
        }
        return false;
    }

    pub fn add_healthpack(&mut self, amount: f32) {
        self.health += amount;

        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }

    pub fn hit(&mut self, damage: f32) -> bool {
        if self.shields > 0.0 {
            self.shields -= damage;
            if self.shields <= 0.0 {
                self.health += self.shields;
                self.shields = 0.0;

                if self.health <= 0.0 {
                    return false;
                }
            }
        }
        return true;
    }
}

pub struct PlayerBox {
    pub angle: f32,
    pub speed: f32,
    pub boost: f32,
    pub col_shape: Rectangle,
    pub system: StructuralSystem,
    pub weapons: Weapons,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum OWNERTYPE {
    Player,
    Enemy,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum BULLETTYPE {
    Regular,
    Spread,
}

pub struct Bullet {
    pub angle: f32,
    pub life_span: f32,
    pub speed: f32,
    pub col_shape: Rectangle,
    pub owner_type: OWNERTYPE,
    pub damage_amount: f32,
}

pub struct Circles {
    pub angle: f32,
    pub life_span: f32,
    pub speed: f32,
}

pub struct CircleController {
    pub spawn_at: f32,
    pub offset: f32,
}

impl CircleController {
    pub fn new() -> CircleController {
        CircleController {
            spawn_at: 0.0,
            offset: 1.0,
        }
    }
}

pub struct Particle {
    pub angle: f32,
    pub speed: f32,
    pub lifetime: f32,
}

pub struct OppoShip {
    pub speed: f32,
    pub angle: f32,
    pub life_span: f32,
    pub col_shape: Rectangle,
    pub level: u32,
    pub system: StructuralSystem,
}

pub struct HiveMind {
    pub level: u32,
    pub spawn_at: f32,
}

impl HiveMind {
    pub fn new() -> HiveMind {
        HiveMind {
            level: 0,
            spawn_at: 0.1,
        }
    }
}

#[derive(Debug)]
pub struct MyWorld {
    pub player: Option<Entity>,
    pub hive_mind: Option<Entity>,
    pub level: u32,
    pub progression: f32,
    pub points: u32,
    pub ship: Option<Entity>,
}

impl MyWorld {
    pub fn new() -> MyWorld {
        MyWorld {
            player: None,
            hive_mind: None,
            level: 1,
            progression: 0.0,
            points: 0,
            ship: None,
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    pub camera: Option<Entity>,
    pub position: (f32, f32),
    pub level: u32,
    pub time_remaining: f32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            camera: None,
            position: (0.0, 0.0),
            level: 0,
            time_remaining: 0.00,
        }
    }
}
