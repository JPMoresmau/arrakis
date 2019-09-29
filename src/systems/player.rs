use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputEvent, StringBindings};
use amethyst::renderer::resources::Tint;
use amethyst::shred::{DynamicSystemData, Resources};
use amethyst::shrev::{EventChannel, ReaderId};
use std::ops::Deref;

use crate::arrakis::{perform_move, move_inhabitants};
use crate::build::{build_zone, show_walls, place_inhabitants};
use crate::components::{Cell, Player, Zone, CellType, Inhabitant, Action};
use crate::config::ArrakisConfig;

pub struct PlayerSystem {
    reader: Option<ReaderId<InputEvent<StringBindings>>>,
}

impl PlayerSystem {
    pub fn new() -> Self {
        Self { reader: None }
    }
}

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Zone>,
        ReadStorage<'s, Cell>,
        WriteStorage<'s, Tint>,
        ReadStorage<'s,Inhabitant>,
        Read<'s, EventChannel<InputEvent<StringBindings>>>,
        Read<'s, ArrakisConfig>,
    );

    fn setup(&mut self, res: &mut Resources) {
        <Self::SystemData as DynamicSystemData>::setup(&self.accessor(), res);
        self.reader = Some(
            res.fetch_mut::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
        );
    }

    fn run(
        &mut self,
        (mut transforms, mut players, mut zones, cells, mut tints, inhabitants, event, config): Self::SystemData,
    ) {
        for event in event.read(self.reader.as_mut().unwrap()) {
            if let InputEvent::ActionPressed(action) = event {
                let mut should_move_inhabitants = false;
                let mut should_place_inhabitants = false;
                let confr=&config.deref();
                for (transform, player, zone) in (&mut transforms, &mut players, &mut zones).join(){
                    let (nz, nx, ny) = match action.as_ref() {
                        "right" => move_right(zone, &config),
                        "left" => move_left(zone, &config),
                        "up" => move_up(zone, &config),
                        "down" => move_down(zone, &config),
                        _ => (zone.current, zone.cell.0, zone.cell.1),
                    };
                    if zone.cell.0 != nx || zone.cell.1 != ny || zone.current != nz {
                       
                        if zone.current != nz {
                            zone.current = nz;
                            zone.cell.0 = nx;
                            zone.cell.1 = ny;
                            build_zone(zone, confr);
                            show_walls(&zone, &cells, &mut tints);
                            perform_move(zone, transform, player, confr);
                            should_place_inhabitants = true;
                            
                        } else {
                            if zone.cells[nx][ny] > CellType::Inhabitant {
                                zone.cell.0 = nx;
                                zone.cell.1 = ny;
                                perform_move(zone, transform, player, confr);
                                should_move_inhabitants = player.action != Some(Action::Charisma);
                            }
                        }
                        player.action=None;
                    } else {
                        match action.as_ref() {
                            "charisma" if player.charisma > 0 => {
                                player.charisma -= 1;
                                player.action=Some(Action::Charisma);
                                },
                            "magic" if player.magic>0 => {
                                player.magic -= 1;
                                player.action=Some(Action::Magic);
                            },
                            "power" => player.action=Some(Action::Power),
                            _ => (),
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
                

            }
        }
    }
}

fn move_right<'s>(zone: &mut Zone, config: &Read<'s, ArrakisConfig>) -> (i32, usize, usize) {
    if zone.cell.0 == config.arena.cell_count - 1 {
        (zone.current + 10, 0, zone.cell.1)
    } else {
        (zone.current, zone.cell.0 + 1, zone.cell.1)
    }
}

fn move_left<'s>(zone: &mut Zone, config: &Read<'s, ArrakisConfig>) -> (i32, usize, usize) {
    if zone.cell.0 == 0 {
        (zone.current - 10, config.arena.cell_count - 1, zone.cell.1)
    } else {
        (zone.current, zone.cell.0 - 1, zone.cell.1)
    }
}

fn move_up<'s>(zone: &mut Zone, config: &Read<'s, ArrakisConfig>) -> (i32, usize, usize) {
    if zone.cell.1 == config.arena.cell_count - 1 {
        (zone.current + 100, zone.cell.0, 0)
    } else {
        (zone.current, zone.cell.0, zone.cell.1 + 1)
    }
}

fn move_down<'s>(zone: &mut Zone, config: &Read<'s, ArrakisConfig>) -> (i32, usize, usize) {
    if zone.cell.1 == 0 {
        (zone.current - 100, zone.cell.0, config.arena.cell_count - 1)
    } else {
        (zone.current, zone.cell.0, zone.cell.1 - 1)
    }
}
