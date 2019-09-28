extern crate rand;

use amethyst::{
    core::transform::Transform,
    };

use crate::build::{set_player_position,is_next_to_inhabitant};
use crate::config::{ArrakisConfig};
use crate::components::{*};



pub fn perform_move(zone: &mut Zone, transform: &mut Transform, player: &mut Player, config: &ArrakisConfig) {
    set_player_position(zone, transform, config);
    player.strength = if player.strength > 0 {
        player.strength - 1
    } else {
        0
    };
    calculate_encounter(zone, player, config);
}

pub fn calculate_encounter(zone: &mut Zone, player: &mut Player, config: &ArrakisConfig) {
    let (x,y) = zone.cell;
    zone.current_type = zone.cells[x][y];
    if !is_next_to_inhabitant(zone,config){
        match zone.current_type {
            CellType::Gold => {
                player.gold += config.gold;
            },
            CellType::Fountain => {
                player.strength = player.strength.max(100);
            },
            CellType::Armourer if player.gold>=config.armourer.gold => {
                player.charisma += config.armourer.charisma;
                player.magic += config.armourer.magic;
                player.gold -= config.armourer.gold;
                },
            CellType::Magician => {
                player.magic += config.magician.magic;
                player.gold += config.magician.gold;
                player.strength += config.magician.strength;
                player.charisma += config.magician.charisma;
            },
            _ => {},
        };
        zone.cells[x][y] = CellType::Empty;
    }
}

