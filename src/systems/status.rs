//! Status system
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};
use amethyst::ui::{UiText};

use crate::components::{Player,Status,Zone,Encounter,CellType};

pub struct StatusSystem;

impl<'s> System<'s> for StatusSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Zone>,
        ReadStorage<'s, Status>,
        ReadStorage<'s, Encounter>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (
        players,
        zones,
        targets,
        encounters,
        mut ui_texts,
    ): Self::SystemData) {
        for (player,zone) in (&players,&zones).join() {
            // player status
            for (_, utext) in (&targets, &mut ui_texts).join(){
                utext.text = format!("{}\n{}\n{}\n{}\n{}\n{}", 
                    player.strength, 
                    player.magic,
                    player.charisma,
                    player.gold,
                    zone.current,
                    zone.target);
            }
            // cell encounter status
            for (_, utext) in (&encounters, &mut ui_texts).join(){
                let s = match zone.current_type {
                    CellType::Fountain => "Fountain",
                    CellType::Armourer => "Armourer",
                    CellType::Magician => "Magician",
                    _ => "",
                };
                utext.text = String::from(s);
            }
        }
    }

}