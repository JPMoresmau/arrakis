extern crate rand;

use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, NullStorage},
    ecs::{Join,ReadStorage,WriteStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture, palette::Srgba,
        resources::Tint,},
    window::ScreenDimensions,
    ui::{Anchor, TtfFormat, UiText, UiTransform, LineMode, FontHandle},
};
use std::ops::Deref;
use rand::Rng;

use crate::config::{ArrakisConfig};

pub struct Arrakis;

impl SimpleState for Arrakis {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let config = {
            let config = &world.read_resource::<ArrakisConfig>();
            config.deref().clone()
            //(config.arena.clone(),config.status.clone())
        };

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
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

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
    pub moving: bool,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            strength: 100,
            magic: 5,
            charisma: 5,
            gold: 100,
            moving: false,
        }
    }

}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug)]
pub struct Zone {
    pub current: i32,
    pub target: i32,
    pub cells: [[bool; 20]; 20],
    pub cell: (usize,usize),
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

pub fn set_player_position(zone: &Zone, transform: &mut Transform, config: &ArrakisConfig){
    transform.set_translation_xyz(zone.cell.0 as f32 * config.cell.width + config.cell.width * 0.5, 
        zone.cell.1 as f32 * config.cell.height + config.cell.height * 0.5,
         0.0);
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