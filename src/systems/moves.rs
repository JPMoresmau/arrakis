//! Move system
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage,Entities, World};
use amethyst::input::{InputEvent, StringBindings};
use amethyst::renderer::{
        resources::Tint,SpriteRender};
use amethyst::shred::{DynamicSystemData};
use amethyst::shrev::{EventChannel, ReaderId};
use std::ops::Deref;

use crate::arrakis::{perform_move, move_inhabitants, clear_shields, add_wizard, need_add_wizard};
use crate::build::{build_zone, show_walls, place_inhabitants};
use crate::components::{Cell, Player, Zone, Inhabitant, Action, CurrentState};
use crate::config::ArrakisConfig;

pub struct MoveSystem {
    reader: Option<ReaderId<InputEvent<StringBindings>>>,
}

impl MoveSystem {
    pub fn new() -> Self {
        Self { reader: None }
    }
}

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Zone>,
        ReadStorage<'s, Cell>,
        WriteStorage<'s, Tint>,
        ReadStorage<'s,Inhabitant>,
        WriteStorage<'s, SpriteRender>,
        Entities<'s>,
        Read<'s, EventChannel<InputEvent<StringBindings>>>,
        Read<'s, ArrakisConfig>,
    );

    /// register event channel
    fn setup(&mut self, w: &mut World) {
        <Self::SystemData as DynamicSystemData>::setup(&self.accessor(), w);
        self.reader = Some(
            w.fetch_mut::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (mut transforms, mut players, mut zones, cells, mut tints, inhabitants, mut sprites, entities, event, config): Self::SystemData,
    ) {
        for event in event.read(self.reader.as_mut().unwrap()) {
            if let InputEvent::ActionPressed(action) = event {
                let mut should_move_inhabitants = false;
                let mut should_place_inhabitants = false;
                let mut should_add_wizard = false;
                let confr=&config.deref();
                for (transform, player, zone) in (&mut transforms, &mut players, &mut zones).join(){
                    if player.current_state ==  CurrentState::Gameplay {
                    
                        let (nz, nx, ny) = match action.as_ref() {
                            "right" => move_right(zone, &config),
                            "left" => move_left(zone, &config),
                            "up" => move_up(zone, &config),
                            "down" => move_down(zone, &config),
                            _ => (zone.current, zone.cell.0, zone.cell.1),
                        };
                        // the action was a move
                        if zone.cell.0 != nx || zone.cell.1 != ny || zone.current != nz {
                            // zone change
                            if zone.current != nz {
                                zone.current = nz;
                                zone.cell.0 = nx;
                                zone.cell.1 = ny;
                                clear_shields(zone, &entities);
                                build_zone(zone, confr);
                                // delete wizard if we leave target zone
                                if zone.current != zone.target {
                                    if let Some(wiz) = zone.wizard.take(){
                                        entities.delete(entities.entity(wiz)).unwrap();
                                    }
                                }
                                should_add_wizard=need_add_wizard(zone);
                                show_walls(zone, &cells, &mut tints);
                                perform_move(zone, transform, player, confr);
                                should_place_inhabitants = true;
                                
                            } else {
                                // check we can move to the cell
                                if zone.cells[nx][ny] < 2 {
                                    zone.cell.0 = nx;
                                    zone.cell.1 = ny;
                                    perform_move(zone, transform, player, confr);
                                    should_move_inhabitants = player.action != Some(Action::Charisma);
                                }
                            }
                            // reset previous action
                            player.action=None;
                        } 
                    }
                }
                    
                if should_place_inhabitants {
                    for (_, zone) in (&mut players, &mut zones).join(){
                        place_inhabitants(zone, &inhabitants, &mut transforms, confr);
                    }
                }
                if should_move_inhabitants {
                    for (_, zone) in (&mut players, &mut zones).join(){
                        move_inhabitants(zone, &inhabitants, &mut transforms, confr);
                    }
                }
                if should_add_wizard {
                    // TODO there must be a better way
                    let mut h = None;
                    for sprite in (&mut sprites).join(){
                        h = Some(sprite.sprite_sheet.clone());
                        break;
                    }
                    let sprite_sheet=&(h.unwrap());
                    for (_, zone) in (&mut players, &mut zones).join(){
                        add_wizard(zone, &entities, sprite_sheet, &mut transforms, &mut sprites, confr);
                    }
                }
            

            }
        }
    }
}

/// move right possibly changing zone
fn move_right<'s>(zone: &mut Zone, config: &Read<'s, ArrakisConfig>) -> (i32, usize, usize) {
    if zone.cell.0 == config.arena.cell_count - 1 {
        (zone.current + 10, 0, zone.cell.1)
    } else {
        (zone.current, zone.cell.0 + 1, zone.cell.1)
    }
}

/// move left possibly changing zone
fn move_left<'s>(zone: &mut Zone, config: &Read<'s, ArrakisConfig>) -> (i32, usize, usize) {
    if zone.cell.0 == 0 {
        (zone.current - 10, config.arena.cell_count - 1, zone.cell.1)
    } else {
        (zone.current, zone.cell.0 - 1, zone.cell.1)
    }
}

/// move up possibly changing zone
fn move_up<'s>(zone: &mut Zone, config: &Read<'s, ArrakisConfig>) -> (i32, usize, usize) {
    if zone.cell.1 == config.arena.cell_count - 1 {
        (zone.current + 100, zone.cell.0, 0)
    } else {
        (zone.current, zone.cell.0, zone.cell.1 + 1)
    }
}

/// move down possibly changing zone
fn move_down<'s>(zone: &mut Zone, config: &Read<'s, ArrakisConfig>) -> (i32, usize, usize) {
    if zone.cell.1 == 0 {
        (zone.current - 100, zone.cell.0, config.arena.cell_count - 1)
    } else {
        (zone.current, zone.cell.0, zone.cell.1 - 1)
    }
}
