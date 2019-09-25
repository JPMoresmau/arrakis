use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputEvent, StringBindings};
use amethyst::renderer::resources::Tint;
use amethyst::shred::{DynamicSystemData, Resources};
use amethyst::shrev::{EventChannel, ReaderId};
use std::ops::Deref;

use crate::arrakis::{build_zone, perform_move, show_walls, Cell, Player, Zone};
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
        (mut transforms, mut players, mut zones, cells, mut tints, event, config): Self::SystemData,
    ) {
        for event in event.read(self.reader.as_mut().unwrap()) {
            if let InputEvent::ActionPressed(action) = event {
                for (transform, player, zone) in (&mut transforms, &mut players, &mut zones).join(){
                    let (nz, nx, ny) = match action.as_ref() {
                        "right" => move_right(zone, &config),
                        "left" => move_left(zone, &config),
                        "up" => move_up(zone, &config),
                        "down" => move_down(zone, &config),
                        _ => (zone.current, zone.cell.0, zone.cell.1),
                    };
                    if zone.cell.0 != nx || zone.cell.1 != ny || zone.current != nz {
                        let confr=&config.deref();
                        if zone.current != nz {
                            zone.current = nz;
                            zone.cell.0 = nx;
                            zone.cell.1 = ny;
                            build_zone(zone, confr);
                            show_walls(&zone, &cells, &mut tints);
                            perform_move(zone, transform, player, confr);

                        } else {
                            if !zone.cells[nx][ny] {
                                zone.cell.0 = nx;
                                zone.cell.1 = ny;
                                perform_move(zone, transform, player, confr);
                            }
                        }
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
