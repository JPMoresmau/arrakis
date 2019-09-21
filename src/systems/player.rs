use amethyst::core::{Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};

use crate::arrakis::{Player};
use crate::config::ArrakisConfig;

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, ArrakisConfig>,
    );

    fn run(&mut self, (mut transforms, mut players, input, config): Self::SystemData) {
        for (transform, player) in (&mut transforms, &mut players).join(){
            if !player.moving {
                if input.action_is_down("right").unwrap_or(false){
                    move_x(transform,config.cell.width, &config);
                    player.moving = true;
                } 
                if input.action_is_down("left").unwrap_or(false){
                    move_x(transform,-config.cell.width, &config);
                    player.moving = true;
                } 
                if input.action_is_down("up").unwrap_or(false){
                    move_y(transform,config.cell.height, &config);
                    player.moving = true;
                } 
                if input.action_is_down("down").unwrap_or(false){
                    move_y(transform,-config.cell.height, &config);
                    player.moving = true;
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

fn move_x<'s>(transform: &mut Transform, amount: f32, config: &Read<'s, ArrakisConfig>){
    let x = transform.translation().x;
    let mut nx = x + amount;
    println!("x: {}, nx: {}",x,nx);
    if nx < config.cell.width {
        nx = config.arena.width;
    } else if nx > config.arena.width {
        nx = config.cell.width;
    }
    transform.set_translation_x(nx);
}

fn move_y<'s>(transform: &mut Transform, amount: f32, config: &Read<'s, ArrakisConfig>){
    let y = transform.translation().y;
    let mut ny = y + amount;
    println!("y: {}, ny: {}",y,ny);
    if ny < 0.0 {
        ny = config.arena.height;
    } else if ny > config.arena.height {
        ny = 0.0;
    }
    transform.set_translation_y(ny);
}