//! Functions to build terrain and entities
extern crate rand;

use crate::components::*;
use crate::config::ArrakisConfig;
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Entity, Join, ReadStorage, WriteStorage},
    prelude::*,
    renderer::{
        palette::Srgba, resources::Tint, Camera, ImageFormat, SpriteRender, SpriteSheet,
        SpriteSheetFormat, Texture,
    },
    ui::{Anchor, FontHandle, LineMode, TtfFormat, UiText, UiTransform},
    window::ScreenDimensions,
};
use rand::seq::SliceRandom;
use rand::Rng;
use std::ops::Deref;

/// load the font for messages
pub fn load_font(world: &mut World) -> FontHandle {
    world
        .read_resource::<Loader>()
        .load("font/square.ttf", TtfFormat, (), &world.read_resource())
}

/// initialize the camera
pub fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    let (width, height) = /*{
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        }; */ (900.0,640.0);
    println!(
        "Screen dimensions on initialize_camera:{},{}",
        width, height
    );
    transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);
    /*{
        let mut config = world.write_resource::<ArrakisConfig>();
        config.cell.width *=  width / 900.0;
        config.cell.height *=  height / 640.0;
    }*/

    world
        .create_entity()
        .with(Camera::standard_2d(width, height))
        .with(transform)
        .build();
}

/// create an entity for each cell to show walls
pub fn initialize_terrain(world: &mut World, sprite_sheet: &Handle<SpriteSheet>) {
    let config = world.read_resource::<ArrakisConfig>().deref().clone();
    for x in 0..20 {
        for y in 0..20 {
            build_wall(x, y, world, sprite_sheet.clone(), &config);
        }
    }
}

/// create inhabitant entities
fn initialize_inhabitants(world: &mut World, sprite_sheet: Handle<SpriteSheet>) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 1,
    };
    let config = world.read_resource::<ArrakisConfig>().deref().clone();
    for _ in 0..config.inhabitants {
        world
            .create_entity()
            .with(Inhabitant::default())
            .with(sprite_render.clone())
            .with(Transform::default())
            .build();
    }
}

/// create player entity and inhabitants of initial zone
pub fn initialize_player(world: &mut World, sprite_sheet: Handle<SpriteSheet>, font: FontHandle) {
    initialize_inhabitants(world, sprite_sheet.clone());

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 2,
    };
    let config = world.read_resource::<ArrakisConfig>().deref().clone();
    let mut rng = rand::thread_rng();

    let n1 = rng.gen_range(0..100);
    let mut zone = Zone {
        current: (n1 + 50) * 10,
        target: 350,
        cells: [[0; 20]; 20],
        cell: (config.arena.cell_count / 2, config.arena.cell_count / 2),
        current_type: CellType::Empty,
        inhabitants: vec![],
        shields: vec![],
        wizard: None,
    };

    build_zone(&mut zone, &config);

    {
        let cells = world.read_storage::<Cell>();
        let mut tints = world.write_storage::<Tint>();
        show_walls(&zone, &cells, &mut tints);
    }
    let mut transform = Transform::default();
    set_player_position(&zone, &mut transform, &config);

    {
        let inhabitants = world.read_storage::<Inhabitant>();
        let mut trs = world.write_storage::<Transform>();
        place_inhabitants(&zone, &inhabitants, &mut trs, &config);
    }

    world
        .create_entity()
        .with(Player {
            charisma: config.player.charisma,
            gold: config.player.gold,
            magic: config.player.magic,
            strength: config.player.strength,
            action: None,
            current_state: CurrentState::Gameplay,
        })
        .with(zone)
        .with(sprite_render)
        .with(transform)
        .build();

    let values_transform = UiTransform::new(
        String::from("StatusValues"),
        Anchor::TopRight,
        Anchor::TopLeft,
        -config.status.values_width,
        0.,
        1.,
        config.status.values_width,
        config.status.height * 0.5,
    );

    let values_uit = UiText::new(
        font.clone(),
        String::new(),
        [1., 1., 1., 1.],
        config.status.font_size,
        LineMode::Wrap,
        Anchor::TopRight,
    );

    world
        .create_entity()
        .with(values_transform)
        .with(values_uit)
        .with(Status)
        .build();

    let encounter_transform = UiTransform::new(
        String::from("EncounterValue"),
        Anchor::TopRight,
        Anchor::TopMiddle,
        -config.status.width,
        -config.status.height * 0.5,
        1.,
        config.status.width,
        config.status.height * 0.5,
    );

    let encounter_uit = UiText::new(
        font.clone(),
        String::new(),
        [1., 1., 1., 1.],
        config.status.font_size,
        LineMode::Single,
        Anchor::TopRight,
    );

    world
        .create_entity()
        .with(encounter_transform)
        .with(encounter_uit)
        .with(Encounter)
        .build();
}

/// set player position via transform
pub fn set_player_position(zone: &Zone, transform: &mut Transform, config: &ArrakisConfig) {
    transform.set_translation_xyz(
        zone.cell.0 as f32 * config.cell.width + config.cell.width * 0.5,
        zone.cell.1 as f32 * config.cell.height + config.cell.height * 0.5,
        0.0,
    );
}

/// load sprite sheeet
pub fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
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

/// create text for player's stats names
pub fn initialize_text(world: &mut World, font: FontHandle) {
    let (names_transform, names_uit) = {
        let status = &world.read_resource::<ArrakisConfig>().status;
        let names_transform = UiTransform::new(
            "StatusNames".to_string(),
            Anchor::TopRight,
            Anchor::TopLeft,
            -status.width,
            0.,
            1.,
            status.text_width,
            status.height * 0.5,
        );
        let names = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            "Strength", "Magic", "Charisma", "Gold", "Current Zone", "Target Zone",
        );

        let names_uit = UiText::new(
            font.clone(),
            names.to_string(),
            [1., 1., 1., 1.],
            status.font_size,
            LineMode::Wrap,
            Anchor::TopLeft,
        );
        (names_transform, names_uit)
    };

    world
        .create_entity()
        .with(names_transform)
        .with(names_uit)
        .build();
}
/*
pub fn is_next_to_inhabitant(zone: &Zone, config: &ArrakisConfig) -> bool {
    let (x,y) = zone.cell;

    for x1 in get_neighbours_range(x,config.arena.cell_count){
        for y1 in get_neighbours_range(y,config.arena.cell_count){
            if x!=x1 || y!=y1 {
                if zone.cells[x1][y1] == CellType::Inhabitant{
                    return true;
                }
            }
        }
    }
    false
}*/

/// build a given zone
pub fn build_zone(zone: &mut Zone, config: &ArrakisConfig) {
    let mut n = 0;

    for i in zone.current..zone.current + 80 {
        // use sinus of zone number to generate walls
        let mut sin = ((i as f32).sin().abs() * 100000.0) as i32;
        for _ in 0..5 {
            let c = sin % 10;
            sin /= 10;
            let y = n / config.arena.cell_count;
            let x = n - (y * config.arena.cell_count);
            n += 1;
            let wall = c < config.arena.wall_threshold && !(x == zone.cell.0 && y == zone.cell.1);
            zone.cells[x][y] = if wall { 2 } else { 0 };
        }
    }
    // ensure we can reach wizard
    if zone.current == zone.target {
        zone.cells[10][10] = 0;
    }

    // put inhabitants in empty spaces
    let mut empties = vec![];
    for x in 0..config.arena.cell_count {
        for y in 0..config.arena.cell_count {
            if zone.cells[x][y] == 0 {
                if x != zone.cell.0 || y != zone.cell.1 {
                    if zone.current != zone.target || x != 10 || y != 10 {
                        empties.push((x, y));
                    }
                }
            }
        }
    }
    zone.inhabitants.clear();
    for (x, y) in empties.choose_multiple(&mut rand::thread_rng(), config.inhabitants) {
        zone.cells[*x][*y] = 18;
        zone.inhabitants.push((*x, *y));
    }
}

/// place inhabitants transform
pub fn place_inhabitants<'s>(
    zone: &Zone,
    inhabitants: &ReadStorage<'s, Inhabitant>,
    positions: &mut WriteStorage<'s, Transform>,
    config: &ArrakisConfig,
) {
    for (&(x, y), (_, transform)) in zone.inhabitants.iter().zip((inhabitants, positions).join()) {
        transform.set_translation_xyz(
            x as f32 * config.cell.width + config.cell.width * 0.5,
            y as f32 * config.cell.height + config.cell.height * 0.5,
            0.0,
        );
    }
}

/// show walls by modifying tints
pub fn show_walls<'s>(
    zone: &Zone,
    cells: &ReadStorage<'s, Cell>,
    tints: &mut WriteStorage<'s, Tint>,
) {
    for (cell, tint) in (cells, tints).join() {
        if zone.cells[cell.position.0][cell.position.1] == 2 {
            tint.0.alpha = 1.0;
        } else {
            tint.0.alpha = 0.0;
        }
    }
}

/// build a wall entity
fn build_wall(
    x: usize,
    y: usize,
    world: &mut World,
    sprite_sheet: Handle<SpriteSheet>,
    config: &ArrakisConfig,
) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        x as f32 * config.cell.width + config.cell.width * 0.5,
        y as f32 * config.cell.height + config.cell.height * 0.5,
        0.0,
    );

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet,
        sprite_number: 0,
    };

    let tint = Tint(Srgba::new(1.0, 1.0, 1.0, 0.0));

    world
        .create_entity()
        .with(Cell { position: (x, y) })
        .with(tint)
        .with(sprite_render)
        .with(transform)
        .build();
}

/// build the text for message screens
pub fn initialize_inter_text(
    world: &mut World,
    font: FontHandle,
    message: &str,
    config: &ArrakisConfig,
    anchor: &Anchor,
    font_ratio: f32,
) -> Entity {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    }; //(900.0,640.0);

    let names_transform = UiTransform::new(
        "EndMessage".to_string(),
        Anchor::Middle,
        Anchor::Middle,
        0.0,
        0.0,
        1.,
        width,
        height,
    );

    let names_uit = UiText::new(
        font.clone(),
        message.to_string(),
        [1., 1., 1., 1.],
        config.status.font_size * font_ratio,
        LineMode::Wrap,
        anchor.clone(),
    );

    world
        .create_entity()
        .with(names_transform)
        .with(names_uit)
        .build()
}
