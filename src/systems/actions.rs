use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage, Entities, World};
use amethyst::input::{InputEvent, StringBindings};
use amethyst::renderer::{
        resources::Tint,SpriteRender};
use amethyst::shred::{DynamicSystemData};
use amethyst::shrev::{EventChannel, ReaderId};
use rand::Rng;

use crate::arrakis::{add_shield, power_clear, };
use crate::build::{show_walls, };
use crate::components::{Cell, Player, Zone, Action, Shield, CurrentState};
use crate::config::ArrakisConfig;

pub struct ActionSystem {
    reader: Option<ReaderId<InputEvent<StringBindings>>>,
}

impl ActionSystem {
    pub fn new() -> Self {
        Self { reader: None }
    }
}

impl<'s> System<'s> for ActionSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Zone>,
        WriteStorage<'s, Transform>, 
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Shield>,
        ReadStorage<'s, Cell>,
        WriteStorage<'s, Tint>,
        Entities<'s>,
        Read<'s, EventChannel<InputEvent<StringBindings>>>,
        Read<'s, ArrakisConfig>,
    );

    fn setup(&mut self, w: &mut World) {
        <Self::SystemData as DynamicSystemData>::setup(&self.accessor(), w);
        self.reader = Some(
            w.fetch_mut::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (mut players, mut zones, mut transforms, mut sprites, mut shields, cells, mut tints, entities, event, config): Self::SystemData,
    ) {
        for event in event.read(self.reader.as_mut().unwrap()) {
            if let InputEvent::ActionPressed(action) = event {
                
                for (player, zone) in (&mut players, &mut zones).join(){
                    if player.current_state ==  CurrentState::Gameplay{
                        match action.as_ref() {
                                "charisma" if player.charisma > 0 => {
                                    player.charisma -= 1;
                                    player.action=Some(Action::Charisma);
                                    },
                                "magic" if player.magic>0 && zone.cells[zone.cell.0][zone.cell.1] == 0 => {
                                    player.magic -= 1;
                                    let mut h = None;
                                    for sprite in (&mut sprites).join(){
                                        h = Some(sprite.sprite_sheet.clone());
                                        break;
                                    }
                                    let sprite_sheet=&(h.unwrap());
                                    add_shield(zone, zone.cell, &entities , sprite_sheet, &mut transforms, &mut sprites, &mut shields, &config);
                                },
                                "power" if player.charisma>9 && player.magic>1 => {
                                    player.strength = player.strength.saturating_sub(5);
                                    player.charisma = player.charisma.saturating_sub(10);
                                    player.magic = player.magic.saturating_sub(1);
                                    let mut rng = rand::thread_rng();
                                    player.gold = player.gold.saturating_sub(rng.gen_range(0, 10) + 15);
                                    power_clear(zone, zone.cell, &config);
                                    show_walls(zone, &cells, &mut tints);
                                },
                                "restart" =>{
                                    player.action = Some(Action::Restart);
                                },
                                "help" =>{
                                    player.action = Some(Action::Help);
                                },
                                _ => (),
                            }
                    } else {
                        match action.as_ref() {
                            // this is used as "resume" on help screen
                            "restart" =>{
                                 player.current_state = CurrentState::Gameplay;
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}