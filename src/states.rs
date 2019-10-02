use amethyst::{ecs::{Join,Entity}, input::*, prelude::*, ui::Anchor};

use crate::build::*;
use crate::components::{Action, Cell, Inhabitant, Player, CurrentState, Zone};
use crate::config::ArrakisConfig;
use std::ops::Deref;
pub struct Arrakis;

impl SimpleState for Arrakis {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        

        world.register::<Cell>();
        world.register::<Inhabitant>();

        let sprite_sheet_handle = load_sprite_sheet(world);

        let font = load_font(world);
        initialize_camera(world);

       
       
        initialize_terrain(world, &sprite_sheet_handle);
        initialize_player(world, sprite_sheet_handle, font.clone());
        initialize_text(world, font);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &data.world;
        for (player,zone) in (&mut world.write_storage::<Player>(),&world.read_storage::<Zone>()).join() {
            if player.strength == 0 {
                return Trans::Switch(Box::new(InterTitle::dead()));
            }
            if let Some(Action::Restart) = player.action {
                player.action = None;
                return Trans::Switch(Box::new(Arrakis));
            }
            if let Some(Action::Help) = player.action {
                player.action = None;
                player.current_state = CurrentState::Intertext;
                return Trans::Push(Box::new(InterTitle::help()));
            }
            if zone.current==zone.target && zone.cell==(10,10) && player.gold>=400 {
                return Trans::Switch(Box::new(InterTitle::success()));
            }
        }

        Trans::None
    }

}

pub struct InterTitle {
    message: String,
    key: VirtualKeyCode,
    restart: bool,
    anchor: Anchor,
    entity: Option<Entity>,
    font_ratio: f32,
}

impl InterTitle {
    pub fn dead() -> InterTitle {
        InterTitle {
            message: "You are DEAD!\nPress R to restart".to_string(),
            key: VirtualKeyCode::R,
            restart: true,
            anchor: Anchor::TopMiddle,
            entity: None,
            font_ratio: 2.0,
        }
    }

    pub fn success() -> InterTitle {
        InterTitle {
            message: "You WIN!\nPress R to have another go".to_string(),
            key: VirtualKeyCode::R,
            restart: true,
            anchor: Anchor::TopMiddle,
            entity: None,
            font_ratio: 2.0,
        }
    }

    pub fn start() -> InterTitle {
        InterTitle {
            message: "Welcome to Arrakis\nPress S to start".to_string(),
            key: VirtualKeyCode::S,
            restart: true,
            anchor: Anchor::TopMiddle,
            entity: None,
            font_ratio: 2.0,
        }
    }

    pub fn help() -> InterTitle {
        InterTitle {
            message: "Arrow keys to move
C to stop inhabitants for one turn
M to put down a magic shield
P to use magic to change terrain
R to restart the game if you're stuck
                
Reach the wizard of Arrakis in zone 350 with 400 Gold
Watch out for special places that will impact your stats
Press R to resume"
                .to_string(),
            key: VirtualKeyCode::R,
            restart: false,
            anchor: Anchor::MiddleLeft,
            entity: None,
            font_ratio: 1.0,
        }
    }
}

impl SimpleState for InterTitle {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        if self.restart{
            world.delete_all();
        }
        let font = load_font(world);

        let config = world.read_resource::<ArrakisConfig>().deref().clone();
        self.entity=Some(initialize_inter_text(world, font, &self.message, &config, &self.anchor, self.font_ratio));
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
                Trans::Quit
            } else if is_key_down(&event, self.key) {
                if let Some(e) = self.entity.take(){
                    data.world.delete_entity(e).unwrap();
                }
                if self.restart {
                    Trans::Switch(Box::new(Arrakis))
                } else {
                    Trans::Pop
                }
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
