use serde::{Deserialize,Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct ArrakisConfig {
    pub arena: ArenaConfig,
    pub cell: CellConfig,
    pub status: StatusConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ArenaConfig {
    pub height: f32,
    pub width: f32,
}

impl Default for ArenaConfig {
    fn default() -> Self {
        ArenaConfig {
            height: 640.0,
            width: 640.0,
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
