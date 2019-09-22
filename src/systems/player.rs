use amethyst::core::{Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::resources::Tint;
use std::ops::Deref;

use crate::arrakis::{Cell, Player, Zone, set_player_position, build_zone, show_walls};
use crate::config::ArrakisConfig;

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Zone>,
        ReadStorage<'s, Cell>,
        WriteStorage<'s, Tint>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, ArrakisConfig>,
    );

    fn run(&mut self, (mut transforms, mut players, mut zones,cells, mut tints, input, config): Self::SystemData) {
        for (transform, player, zone) in (&mut transforms, &mut players, &mut zones).join(){
            if !player.moving {
                let (nz, nx, ny) = if input.action_is_down("right").unwrap_or(false){
                        move_right(zone, &config)
                    } else if input.action_is_down("left").unwrap_or(false){
                        move_left(zone, &config)
                    } else if input.action_is_down("up").unwrap_or(false){
                        move_up(zone, &config)
                    } else if input.action_is_down("down").unwrap_or(false){
                        move_down(zone, &config)
                    } else {
                        (zone.current,zone.cell.0,zone.cell.1)
                    };
                if zone.cell.0!=nx || zone.cell.1!=ny || zone.current!=nz {
                    player.moving = true;
                    if zone.current!=nz {
                        zone.current = nz;
                        zone.cell.0 = nx;
                        zone.cell.1 = ny;
                        build_zone(zone, &config.deref());
                        show_walls(&zone, &cells, &mut tints);
                        set_player_position(zone, transform, &config.deref());
                    } else {
                        if !zone.cells[nx][ny] {
                            zone.cell.0 = nx;
                            zone.cell.1 = ny;
                            set_player_position(zone, transform, &config.deref());
                        }
                    }
                }
            } else {
                player.moving = input.action_is_down("right").unwrap_or(false)
                    || input.action_is_down("left").unwrap_or(false)
                    || input.action_is_down("up").unwrap_or(false)
                    || input.action_is_down("down").unwrap_or(false);
                
            }
        }
    }
    
}


fn move_right<'s>(zone: &mut Zone, config: &Read<'s, ArrakisConfig>) -> (i32, usize,usize){
    if zone.cell.0 == config.arena.cell_count-1 {
            (zone.current+10 ,0 , zone.cell.1)
    } else {
            (zone.current, zone.cell.0 + 1,zone.cell.1)
    }
}

fn move_left<'s>(zone: &mut Zone, config: &Read<'s, ArrakisConfig>) -> (i32,usize,usize){
    if zone.cell.0 == 0 {
            (zone.current-10 ,config.arena.cell_count - 1 , zone.cell.1)
    } else {
            (zone.current, zone.cell.0 - 1, zone.cell.1)
    }
}

fn move_up<'s>(zone: &mut Zone, config: &Read<'s, ArrakisConfig>) -> (i32,usize,usize){
    if zone.cell.1 == config.arena.cell_count-1 {
        (zone.current+100, zone.cell.0 ,0 )
    } else {
        (zone.current, zone.cell.0, zone.cell.1 + 1)
    }
}

fn move_down<'s>(zone: &mut Zone, config: &Read<'s, ArrakisConfig>) -> (i32,usize,usize){
    if zone.cell.1 == 0 {
        (zone.current-100, zone.cell.0 ,config.arena.cell_count - 1)
    } else {
        (zone.current, zone.cell.0, zone.cell.1 - 1)
    }
}
