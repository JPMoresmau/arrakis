extern crate rand;

use amethyst::{
    core::transform::Transform,
    ecs::{ReadStorage,WriteStorage},
    };

use crate::build::{set_player_position,is_next_to_inhabitant, place_inhabitants, set_cell_type};
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

pub fn move_inhabitants<'s>(zone: &mut Zone,inhabitants: &ReadStorage<'s,Inhabitant>,positions: &mut WriteStorage<'s,Transform>, config: &ArrakisConfig) {
    
    zone.inhabitants = zone.inhabitants.clone().iter().map(|pos| move_inhabitant(zone,pos,config)).collect();

    place_inhabitants(zone, inhabitants, positions, config);
}

fn move_inhabitant(zone : &mut Zone, pos: &(usize, usize), config: &ArrakisConfig) -> (usize,usize) {
    let (xp,yp) = zone.cell;
    let (x,y) = *pos;
    if x<=xp && y<=yp && x<config.arena.cell_count-1 && y<config.arena.cell_count-1 && (x+1,y+1)!=(xp,yp) && zone.cells[x+1][y+1] > CellType::Inhabitant {
        return set_inhabitant_zone(zone,pos,(x+1,y+1),config);
    }
    if x<=xp && y>=yp && x<config.arena.cell_count-1 && y>0 && (x+1,y-1)!=(xp,yp) && zone.cells[x+1][y-1] > CellType::Inhabitant {
        return set_inhabitant_zone(zone,pos,(x+1,y-1),config);
    }
    if x>=xp && y>=yp && x>0 && y>0 && (x-1,y-1)!=(xp,yp) && zone.cells[x-1][y-1] > CellType::Inhabitant {
        return set_inhabitant_zone(zone,pos,(x-1,y-1),config);
    }
    if x>=xp && y<=yp && x>0 && (x-1,y+1)!=(xp,yp) && y<config.arena.cell_count-1 && zone.cells[x-1][y+1] > CellType::Inhabitant {
        return set_inhabitant_zone(zone,pos,(x-1,y+1),config);
    }
    *pos
}

fn set_inhabitant_zone(zone : &mut Zone, pos: &(usize, usize),new_pos:(usize,usize), config: &ArrakisConfig) -> (usize,usize){
    set_cell_type(zone,pos,config);
    zone.cells[new_pos.0][new_pos.1] = CellType::Inhabitant;
    new_pos
}