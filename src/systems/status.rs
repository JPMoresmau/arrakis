use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};
use amethyst::ui::{UiText};

use crate::arrakis::{Player,Status,Zone};

pub struct StatusSystem;

impl<'s> System<'s> for StatusSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Zone>,
        ReadStorage<'s, Status>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (
        players,
        zones,
        targets,
        mut ui_texts,
    ): Self::SystemData) {
        for (player,zone) in (&players,&zones).join() {
            for (_, utext) in (&targets, &mut ui_texts).join(){
                utext.text = format!("{}\n{}\n{}\n{}\n{}\n{}", 
                    player.strength, 
                    player.magic,
                    player.charisma,
                    player.gold,
                    zone.current,
                    zone.target);
            }
        }
    }

}