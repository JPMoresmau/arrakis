use amethyst::{
    ecs::prelude::{Component, DenseVecStorage, NullStorage},
};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Action {
    Charisma,
    Magic,
    Power,
}

#[derive(Debug)]
pub struct Player {
    pub strength: u32,
    pub magic: u32,
    pub charisma: u32,
    pub gold: u32,
    pub action: Option<Action>,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            strength: 100,
            magic: 5,
            charisma: 5,
            gold: 100,
            action: None,
        }
    }

}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum CellType {
    Wall,
    Inhabitant,
    Empty,
    Fountain,
    Armourer,
    Magician,
    Gold,
}

#[derive(Debug)]
pub struct Zone {
    pub current: i32,
    pub target: i32,
    pub cells: [[CellType; 20]; 20],
    pub cell: (usize,usize),
    pub current_type: CellType,
    pub inhabitants: Vec<(usize,usize)>,
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
pub struct Encounter;

impl Component for Encounter {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Cell{
    pub position: (usize,usize),
}

impl Component for Cell {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Inhabitant {
}

impl Component for Inhabitant {
    type Storage = NullStorage<Self>;
}

