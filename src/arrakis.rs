extern crate rand;

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, NullStorage},
    ecs::{Join,ReadStorage,WriteStorage},
    input::{*},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, palette::Srgba,
        resources::Tint,},
    window::ScreenDimensions,
    ui::{Anchor, TtfFormat, UiText, UiTransform, LineMode, FontHandle},
};

use rand::Rng;

use crate::config::{ArrakisConfig};

pub struct Arrakis;

impl SimpleState for Arrakis {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.delete_all();

        let config = &world.read_resource::<ArrakisConfig>().clone();

        world.register::<Cell>();

        let sprite_sheet_handle = load_sprite_sheet(world);

        let font = world.read_resource::<Loader>().load(
            "font/square.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

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

        let font = world.read_resource::<Loader>().load(
            "font/square.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

        let config = &world.read_resource::<ArrakisConfig>().clone();
        
        initialize_end_text(world, font, &self.message, &config);
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



fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    let (width, height) = /*{
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        }*/ (900.0,640.0);
    //println!("Screen dimensions on initialize_camera:{},{}",width,height);
    transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}

#[derive(Debug)]
pub struct Player {
    pub strength: u32,
    pub magic: u32,
    pub charisma: u32,
    pub gold: u32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            strength: 100,
            magic: 5,
            charisma: 5,
            gold: 100,
        }
    }

}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub enum Encounter {
    Fountain,
    Armourer,
    Magician,
    Gold,
}

#[derive(Debug)]
pub struct Zone {
    pub current: i32,
    pub target: i32,
    pub cells: [[bool; 20]; 20],
    pub cell: (usize,usize),
    pub encounter: Option<Encounter>,
}

impl Component for Zone {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Status;

impl Component for Status {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Cell{
    pub position: (usize,usize),
}

impl Component for Cell {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_terrain(world: &mut World, sprite_sheet: &Handle<SpriteSheet>, config: &ArrakisConfig) {

    for x in 0..20 {
        for y in 0..20 {
            build_wall(x, y, world, sprite_sheet.clone(), config);
        }
    }

}

fn initialize_player(world: &mut World, sprite_sheet: Handle<SpriteSheet>, font: FontHandle, config: &ArrakisConfig) {
   
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 2, 
    };

    let mut rng = rand::thread_rng();

    let n1 = rng.gen_range(0, 100);
    let mut zone = Zone {
        current: (n1+50)*10,
        target: 350,
        cells: [[false; 20]; 20],
        cell: (config.arena.cell_count /2 , config.arena.cell_count / 2),
        encounter: None
    };
    
    build_zone(&mut zone, config);
       
    {
        let cells = world.read_storage::<Cell>();
        let mut tints = world.write_storage::<Tint>();
        show_walls(&zone, &cells, &mut tints);
    }
    let mut transform = Transform::default();
    set_player_position(&zone, &mut transform, config);
  

    world.create_entity()
        .with(Player::default())
        .with(zone)
        .with(sprite_render)
        .with(transform)
        .build();

    let values_transform = UiTransform::new(
        "StatusValues".to_string(), Anchor::TopRight, Anchor::TopMiddle,
        -config.status.values_width, 0., 1., config.status.values_width, config.status.height,
    );

    let mut values_uit = UiText::new(
            font.clone(),
            "names".to_string(),
            [1., 1., 1., 1.],
            config.status.font_size,
        );
    values_uit.line_mode=LineMode::Wrap;
    values_uit.align=Anchor::TopRight;

    world
        .create_entity()
        .with(values_transform)
        .with(values_uit)
        .with(Status)
        .build();
}

fn set_player_position(zone: &Zone, transform: &mut Transform, config: &ArrakisConfig){
    transform.set_translation_xyz(zone.cell.0 as f32 * config.cell.width + config.cell.width * 0.5, 
        zone.cell.1 as f32 * config.cell.height + config.cell.height * 0.5,
         0.0);
}

pub fn perform_move(zone: &mut Zone, transform: &mut Transform, player: &mut Player, config: &ArrakisConfig) {
    set_player_position(zone, transform, config);
    player.strength = if player.strength > 0 {
        player.strength - 1
    } else {
        0
    };
    calculate_encounter(zone, player, config);
}

pub fn calculate_encounter(zone: &mut Zone, player: &mut Player, config: &ArrakisConfig) {
    let (x,y) = zone.cell;
    let mut sc = 0;
    for x1 in get_neighbours_range(x,config.arena.cell_count){
        for y1 in get_neighbours_range(y,config.arena.cell_count){
            if x!=x1 || y!=y1 {
                if zone.cells[x1][y1] {
                    sc += 1;
                }
            }
        }
    }
    let e = match sc {
        5 => Some(Encounter::Fountain),
        6 => Some(Encounter::Armourer),
        7 => Some(Encounter::Magician),
        _ => None,
    };
    zone.encounter = e;
}

fn get_neighbours_range(x: usize, cell_count: usize) -> Vec<usize> {
    if x > 0 {
        if x < cell_count {
            vec!(x-1, x, x+1)
        } else {
             vec!(x-1, x)
        }
    } else {
        vec!(x , x + 1)
    }
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

     
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/spritesheet.ron", 
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_text(world: &mut World, font: FontHandle, config: &ArrakisConfig) {

    let names_transform = UiTransform::new(
        "StatusNames".to_string(), Anchor::TopRight, Anchor::TopMiddle,
        -config.status.width, 0., 1., config.status.text_width, config.status.height,
    );
    let names = format!("{}\n{}\n{}\n{}\n{}\n{}", 
                    "Strength", 
                    "Magic",
                    "Charisma",
                    "Gold",
                    "Current Zone",
                    "Target Zone",
                    );

    let mut names_uit = UiText::new(
            font.clone(),
            names.to_string(),
            [1., 1., 1., 1.],
            config.status.font_size,
        );
    names_uit.line_mode=LineMode::Wrap;
    names_uit.align=Anchor::TopLeft;


    world
        .create_entity()
        .with(names_transform)
        .with(names_uit)
        .build();
}

pub fn build_zone(zone: &mut Zone, config: &ArrakisConfig){
    let mut n = 0;

    for i in zone.current..zone.current+80 {
        let mut sin = ((i as f32).sin().abs() * 100000.0) as i32;
        // println!("sin: {}", sin);
        for _ in 0..5 {
            let c = sin % 10;
            sin /= 10;
            let y = n / config.arena.cell_count;
            let x = n - (y * config.arena.cell_count);
            n += 1;
            let wall = c < config.arena.wall_threshold && !(x == zone.cell.0 && y == zone.cell.1);
            //  println!("x: {}, y: {}, wall: {}", x, y, wall);
            zone.cells[x][y] = wall;
        }
    }
    
}

pub fn show_walls<'s>(zone: &Zone, cells: &ReadStorage<'s,Cell>, tints: &mut WriteStorage<'s,Tint>){
    for (cell,tint) in (cells,tints).join(){
        if zone.cells[cell.position.0][cell.position.1]{
            tint.0.alpha = 1.0;
        } else {
            tint.0.alpha = 0.0;
        }
    }
}

fn build_wall(x: usize, y: usize, world: &mut World, sprite_sheet: Handle<SpriteSheet>, config: &ArrakisConfig){
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        x as f32 * config.cell.width + config.cell.width * 0.5, 
        y as f32 * config.cell.height + config.cell.height * 0.5, 
        0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 0, 
    };

    let tint = Tint(Srgba::new(1.0, 1.0, 1.0, 0.0));

    world.create_entity()
        .with(Cell {
            position: (x,y),
        })
        .with(tint)
        .with(sprite_render)
        .with(transform)
        .build();
}

fn initialize_end_text(world: &mut World, font: FontHandle, message: &str, config: &ArrakisConfig) {
    let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

    let names_transform = UiTransform::new(
        "EndMessage".to_string(), Anchor::Middle, Anchor::TopMiddle,
        0.0, 0.0, 1., width, height
    );

    let mut names_uit = UiText::new(
            font.clone(),
            message.to_string(),
            [1., 1., 1., 1.],
            config.status.font_size * 2.0,
        );
    names_uit.line_mode=LineMode::Wrap;
    names_uit.align=Anchor::TopMiddle;


    world
        .create_entity()
        .with(names_transform)
        .with(names_uit)
        .build();
}