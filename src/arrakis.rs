//! Some important action handling functions
extern crate rand;

use amethyst::{
    assets::{Handle},
    core::transform::Transform,
    ecs::{ReadStorage,WriteStorage,Entities},
    renderer::{ SpriteRender, SpriteSheet,},
    };

use crate::build::{set_player_position,place_inhabitants};
use crate::config::{ArrakisConfig};
use crate::components::{*};


/// move the player to a new position, and calculate if the cell is special (encounter a fountain, armourer, magician or gold)
pub fn perform_move(zone: &mut Zone, transform: &mut Transform, player: &mut Player, config: &ArrakisConfig) {
    set_player_position(zone, transform, config);
    player.strength = if player.strength > 0 {
        player.strength - 1
    } else {
        0
    };
    calculate_encounter(zone, player, config);
}

/// calculate if the current cell is a special encounter and apply changes
pub fn calculate_encounter(zone: &mut Zone, player: &mut Player, config: &ArrakisConfig) {
    let (x,y) = zone.cell;
    zone.current_type = get_cell_type(zone, &(x,y), config);
    
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
    
}

/// get the encounter cell type
pub fn get_cell_type(zone: &mut Zone, pos: &(usize,usize), config: &ArrakisConfig) -> CellType {
    let (x,y) = *pos;
    let mut sc = 0;
    for x1 in get_neighbours_range(x,config.arena.cell_count){
        for y1 in get_neighbours_range(y,config.arena.cell_count){
            if x!=x1 || y!=y1 {
                sc += zone.cells[x1][y1];
            }
        }
    }
    match sc {
        0 => CellType::Gold,
        10 => CellType::Fountain,
        12 => CellType::Armourer,
        14 => CellType::Magician,
        _ => CellType::Empty,
    }
}

/// get the valid range of cell in the zone (so stopping at zone boundaries)
fn get_neighbours_range(x: usize, cell_count: usize) -> Vec<usize> {
    if x > 0 {
        if x < cell_count-1 {
            vec!(x-1, x, x+1)
        } else {
             vec!(x-1, x)
        }
    } else {
        vec!(x , x + 1)
    }
}

/// get the valid range of cells for power action (doesn't include middle cell)
fn get_power_range(x: usize, cell_count: usize) -> Vec<usize> {
    if x > 0 {
        if x < cell_count-1 {
            vec!(x-1, x+1)
        } else {
             vec!(x-1)
        }
    } else {
        vec!(x + 1)
    }
}

/// clear cells in power action
pub fn power_clear(zone: &mut Zone, pos: (usize, usize), config: &ArrakisConfig){
    for x in get_power_range(pos.0, config.arena.cell_count){
        zone.cells[x][pos.1] = 0;
    }
    for y in get_power_range(pos.1, config.arena.cell_count){
        zone.cells[pos.0][y] = 0;
    }
    
}

/// move inhabitants
pub fn move_inhabitants<'s>(zone: &mut Zone,inhabitants: &ReadStorage<'s,Inhabitant>,positions: &mut WriteStorage<'s,Transform>, config: &ArrakisConfig) {
    
    zone.inhabitants = zone.inhabitants.clone().iter().map(|pos| move_inhabitant(zone,pos,config)).collect();

    place_inhabitants(zone, inhabitants, positions, config);
}

/// move an inhabitant from the given cell towards the player
fn move_inhabitant(zone : &mut Zone, pos: &(usize, usize), config: &ArrakisConfig) -> (usize,usize) {
    let (xp,yp) = zone.cell;
    let (x,y) = *pos;
    if x<=xp && y<=yp && x<config.arena.cell_count-1 && y<config.arena.cell_count-1 && can_move_to(zone,x+1,y+1) {
        return set_inhabitant_cell(zone,pos,(x+1,y+1),config);
    }
    if x<=xp && y>=yp && x<config.arena.cell_count-1 && y>0 && can_move_to(zone,x+1,y-1) {
        return set_inhabitant_cell(zone,pos,(x+1,y-1),config);
    }
    if x>=xp && y>=yp && x>0 && y>0 && can_move_to(zone,x-1,y-1) {
        return set_inhabitant_cell(zone,pos,(x-1,y-1),config);
    }
    if x>=xp && y<=yp && x>0 && y<config.arena.cell_count-1 && can_move_to(zone,x-1,y+1) {
        return set_inhabitant_cell(zone,pos,(x-1,y+1),config);
    }
    *pos
}

/// can an inhabitant move to the given zone
fn can_move_to(zone: &Zone, x: usize, y: usize) ->bool{
    let (xp,yp) = zone.cell;
    (x,y)!=(xp,yp) && zone.cells[x][y] == 0 && (zone.current != zone.target || x!=10 || y!=10)
}

/// move the inhabitant from one cell to another
fn set_inhabitant_cell(zone : &mut Zone, pos: &(usize, usize),new_pos:(usize,usize), _config: &ArrakisConfig) -> (usize,usize){
    zone.cells[pos.0][pos.1] = 0;
    zone.cells[new_pos.0][new_pos.1] = 18;
    new_pos
}

/// add a shield to the given position
pub fn add_shield<'s>(zone: &mut Zone, pos: (usize,usize), entities: &Entities<'s>, sprite_sheet: &Handle<SpriteSheet>, 
    transforms: &mut WriteStorage<'s, Transform>, sprites: &mut WriteStorage<'s, SpriteRender>, shields: &mut WriteStorage<'s,Shield>,
    config: &ArrakisConfig){
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        pos.0 as f32 * config.cell.width + config.cell.width * 0.5, 
        pos.1 as f32 * config.cell.height + config.cell.height * 0.5, 
        0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 4, 
    };
  
    let shield = entities.build_entity()
        .with(transform, transforms)
        .with(sprite_render,sprites)
        .with(Shield{position:pos,}, shields)
        .build();
   
    zone.cells[pos.0][pos.1] = 1;
    zone.shields.push(shield.id());
}

/// do we need to add the wizard entity (target zone and wizard entity doesn't exist)
pub fn need_add_wizard(zone: &Zone) -> bool {
    zone.current == zone.target && zone.wizard.is_none()
}

/// add the wizard entity
pub fn add_wizard<'s>(zone: &mut Zone, entities: &Entities<'s>, sprite_sheet: &Handle<SpriteSheet>, 
    transforms: &mut WriteStorage<'s, Transform>, sprites: &mut WriteStorage<'s, SpriteRender>, config: &ArrakisConfig ){
        if need_add_wizard(zone) {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                10.0 as f32 * config.cell.width + config.cell.width * 0.5, 
                10.0 as f32 * config.cell.height + config.cell.height * 0.5, 
                0.0);

            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: 3, 
            };

            let wizard = entities.build_entity()
                .with(transform, transforms)
                .with(sprite_render,sprites)
                .build();
            zone.wizard = Some(wizard.id());
            
               
        } 
}

/// clear all shields
pub fn clear_shields<'s>(zone: &mut Zone, entities: &Entities<'s>) {
    while let Some(id) = zone.shields.pop() {
        entities.delete(entities.entity(id)).unwrap();
    }
}