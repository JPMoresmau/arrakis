use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, Entity, NullStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    window::ScreenDimensions,
    ui::{Anchor, TtfFormat, UiText, UiTransform, LineMode, FontHandle},
};

pub const ARENA_HEIGHT: f32 = 640.0;
pub const ARENA_WIDTH: f32 = 640.0;

pub const STATUS_HEIGHT: f32 = 640.0;
pub const STATUS_WIDTH: f32 = 240.0;

pub const CELL_HEIGHT: f32 = 32.0;
pub const CELL_WIDTH: f32 = 32.0;


pub struct Arrakis;

impl SimpleState for Arrakis {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        let font = world.read_resource::<Loader>().load(
            "font/square.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

        initialize_camera(world);
        initialize_player(world, sprite_sheet_handle, font.clone());
        initialize_text(world, font);

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

#[derive(Default)]
pub struct Status;

impl Component for Status {
    type Storage = NullStorage<Self>;
}


fn initialize_player(world: &mut World, sprite_sheet: Handle<SpriteSheet>, font: FontHandle) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT *0.5, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 2, 
    };


    world.create_entity()
        .with(Player::default())
        .with(sprite_render.clone())
        .with(transform)
        .build();

    let values_transform = UiTransform::new(
        "StatusValues".to_string(), Anchor::TopRight, Anchor::TopMiddle,
        -STATUS_WIDTH * 0.2, 0., 1., STATUS_WIDTH * 0.2, STATUS_HEIGHT,
    );

    let mut values_uit = UiText::new(
            font.clone(),
            "names".to_string(),
            [1., 1., 1., 1.],
            32.,
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

fn initialize_text(world: &mut World, font: FontHandle) {

    let names_transform = UiTransform::new(
        "StatusNames".to_string(), Anchor::TopRight, Anchor::TopMiddle,
        -STATUS_WIDTH, 0., 1., STATUS_WIDTH * 0.8, STATUS_HEIGHT,
    );
    let names = format!("{}\n{}\n{}\n{}", 
                    "Strength", 
                    "Magic",
                    "Charisma",
                    "Gold",);

    let mut names_uit = UiText::new(
            font.clone(),
            names.to_string(),
            [1., 1., 1., 1.],
            32.,
        );
    names_uit.line_mode=LineMode::Wrap;
    names_uit.align=Anchor::TopLeft;


    world
        .create_entity()
        .with(names_transform)
        .with(names_uit)
        .build();
}