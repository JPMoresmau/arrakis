use serde::{Deserialize,Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct ArrakisConfig {
    pub arena: ArenaConfig,
    pub cell: CellConfig,
    pub status: StatusConfig,
    pub gold: u32,
    pub armourer: ArmourerConfig,
    pub magician: MagicianConfig,
    pub inhabitants: usize,
    pub player: PlayerConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArenaConfig {
    pub height: f32,
    pub width: f32,
    pub wall_threshold: i32,
    pub cell_count: usize,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        ArenaConfig {
            height: 640.0,
            width: 640.0,
            wall_threshold: 5,
            cell_count: 20,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CellConfig {
    pub height: f32,
    pub width: f32,
}

impl Default for CellConfig {
    fn default() -> Self {
        CellConfig {
            height: 32.0,
            width: 32.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StatusConfig {
    pub height: f32,
    pub width: f32,
    pub values_width: f32,
    pub text_width: f32,
    pub font_size: f32,
}

impl Default for StatusConfig {
    fn default() -> Self {
        StatusConfig {
            height: 640.0,
            width: 640.0,
            values_width: 40.0,
            text_width: 200.0,
            font_size: 28.0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArmourerConfig {
    pub charisma: u32,
    pub magic: u32,
    pub gold: u32,
}

impl Default for ArmourerConfig {
    fn default() -> Self {
        ArmourerConfig {
            charisma: 10,
            magic: 1,
            gold: 20,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MagicianConfig {
    pub charisma: u32,
    pub magic: u32,
    pub gold: u32,
    pub strength: u32,
}

impl Default for MagicianConfig {
    fn default() -> Self {
        MagicianConfig {
            magic: 10,
            gold: 20,
            strength: 50,
            charisma: 20,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerConfig {
    pub charisma: u32,
    pub magic: u32,
    pub gold: u32,
    pub strength: u32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        PlayerConfig {
            magic: 5,
            gold: 100,
            strength: 100,
            charisma: 5,
        }
    }
}

