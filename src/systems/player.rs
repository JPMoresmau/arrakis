use amethyst::core::{Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};

use crate::arrakis::{Player,ARENA_HEIGHT, ARENA_WIDTH, CELL_HEIGHT, CELL_WIDTH};

pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, mut players, input): Self::SystemData) {
        for (transform, player) in (&mut transforms, &mut players).join(){
            if !player.moving {
                if input.action_is_down("right").unwrap_or(false){
                    move_x(transform,CELL_WIDTH);
                    player.moving = true;
                } 
                if input.action_is_down("left").unwrap_or(false){
                    move_x(transform,-CELL_WIDTH);
                    player.moving = true;
                } 
                if input.action_is_down("up").unwrap_or(false){
                    move_y(transform,CELL_HEIGHT);
                    player.moving = true;
                } 
                if input.action_is_down("down").unwrap_or(false){
                    move_y(transform,-CELL_HEIGHT);
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

fn move_x(transform: &mut Transform, amount: f32){
    let x = transform.translation().x;
    let mut nx = x + amount;
    println!("x: {}, nx: {}",x,nx);
    if nx < CELL_WIDTH {
        nx = ARENA_WIDTH;
    } else if nx > ARENA_WIDTH {
        nx = CELL_WIDTH;
    }
    transform.set_translation_x(nx);
}

fn move_y(transform: &mut Transform, amount: f32){
    let y = transform.translation().y;
    let mut ny = y + amount;
    println!("y: {}, ny: {}",y,ny);
    if ny < 0.0 {
        ny = ARENA_HEIGHT;
    } else if ny > ARENA_HEIGHT {
        ny = 0.0;
    }
    transform.set_translation_y(ny);
}