use amethyst::{
    ecs::{Join},
    input::{*},
    prelude::*,
};

use crate::build::{*};
use crate::config::{ArrakisConfig};
use crate::components::{Cell, Player, Inhabitant};

pub struct Arrakis;

impl SimpleState for Arrakis {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        let config = &world.read_resource::<ArrakisConfig>().clone();

        world.register::<Cell>();
        world.register::<Inhabitant>();

        let sprite_sheet_handle = load_sprite_sheet(world);

        let font = load_font(world);
        initialize_camera(world);
        initialize_terrain(world, &sprite_sheet_handle, &config);
        initialize_player(world, sprite_sheet_handle, font.clone(), &config);
        initialize_text(world, font, &config);

    }

   fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
       let world = &data.world;
       for player in world.read_storage::<Player>().join(){
           if player.strength == 0 {
               return Trans::Switch(Box::new(InterTitle::dead()));
           }
       }

       Trans::None
    }

}



pub struct InterTitle {
    message: String,
    key: VirtualKeyCode,
}

impl InterTitle {
    pub fn dead() -> InterTitle  {
        InterTitle{
            message:"You are DEAD!\nPress R to restart".to_string(),
            key: VirtualKeyCode::R,
        }
    }

    pub fn start() -> InterTitle  {
        InterTitle {
            message:"Welcome to Arrakis\nPress S to start".to_string(),
            key: VirtualKeyCode::S,
        }
    }
}

impl SimpleState for InterTitle {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        let font = load_font(world);

        let config = &world.read_resource::<ArrakisConfig>().clone();
        
        initialize_inter_text(world, font, &self.message, &config);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
                Trans::Quit
            } else if is_key_down(&event, self.key){
                Trans::Switch(Box::new(Arrakis))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
    
}
